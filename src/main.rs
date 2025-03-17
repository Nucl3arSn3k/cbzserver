use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::string;

use actix_web::http::header::ContentType;
use actix_web::HttpRequest;
use actix_web::{get, http::StatusCode, post, web::Json, App, HttpResponse, HttpServer};
use cbztools::{cHold, catalog_dir, dbconfig};
use matchlogic::match_logic;
use rusqlite::{Connection, Result};
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use treegen::{create_graph, dump_graph};
use petgraph::graph::{Graph, NodeIndex};
use std::path::{Path, PathBuf};
mod cbztools;
mod matchlogic;
mod sqlitejson;
mod treegen;
#[derive(Deserialize)]
struct FilePath {
    filepath: String,
}

#[derive(Deserialize)]
struct Login {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginPerms {
    sucess: bool,
    token: Option<String>,
}

#[derive(Serialize)]
struct Library {
    series: Vec<cHold>,
}

#[get("/api/library")]
async fn library_send() -> HttpResponse {
    let now = std::time::Instant::now();

    let dbcon = dbconfig("cache.db".to_string());
    let mut vax = Vec::new();

    if dbcon == false {
        println!("Starting scan");
        vax = catalog_dir(Path::new("I:\\Comics"), false).await; //Insert an arbiter here?

        let e_time = now.elapsed();
        let min_val = e_time.as_secs() / 60;
        let h_val = min_val / 60;
        let stval = format!(
            "Scan took {} seconds, which is {} minutes and is {} hours",
            e_time.as_secs(),
            min_val,
            h_val
        );
        println!("{}", stval);
        if let Err(e) =
            File::create("timestamp.txt").and_then(|mut file| file.write(stval.as_bytes()))
        {
            println!("Error is {}", e);
            return HttpResponse::InternalServerError()
                .content_type(ContentType::plaintext())
                .insert_header(("logfile writing error", "demo"))
                .finish();
        }

        // Function continues here with the database connection...

        let connection = match Connection::open("cache.db") {
            Ok(conn) => conn,
            Err(e) => {
                println!("Error is {}", e);
                return HttpResponse::InternalServerError()
                    .content_type(ContentType::plaintext())
                    .insert_header(("SQLITE server error", "demo"))
                    .finish(); // Return early from the function with an empty Vec
            }
        };

        for lochold in vax {
            match connection.execute(
                "INSERT INTO files (name, filepath, coverpath, dirornot) VALUES (?1, ?2, ?3, ?4)",
                (
                    &lochold.name,
                    &lochold.filepath.to_string_lossy().to_string(),
                    &lochold
                        .cover_path
                        .as_ref()
                        .map(|p| p.to_string_lossy().to_string()),
                    &lochold.dirornot,
                ),
            ) {
                Ok(_) => (),
                Err(e) => {
                    println!("Error {}", e);
                }
            }
        }
    } else {
        //Just print something,idk.
        println!("DB found")
    }
    let connection = match Connection::open("cache.db") {
        Ok(conn) => conn,
        Err(e) => {
            println!("Error is {}", e);
            return HttpResponse::InternalServerError().body(format!("Error: {}", e)); // Return early from the function with an empty Vec
        }
    };
    //Pass connection to treegen 
    let graph = create_graph(connection);
    
    dump_graph(graph.tree);
    
    /* 
    let stval = match sqlitejson::sq_to_json_boxed(connection) {
        Ok(strval) => {
            println!("{}", strval);
            strval
        }
        Err(e) => {
            println!("Error is {}", e);
            return HttpResponse::InternalServerError().body(format!("Error: {}", e));
        }
    };*/

    /*
    match serde_json::to_string_pretty(&vax) {
        Ok(serialized) => println!("Serialized data:\n{}", serialized),
        Err(e) => println!("Serialization error: {}", e),
    }
    */
    //HttpResponse::Ok().json(vax)
    let stvaltmp = "test";
    return HttpResponse::Ok().json(stvaltmp)
        
}

#[post("/api/login")]
async fn logincheck(credentials: Json<Login>) -> HttpResponse {
    if credentials.username == "blarch" && credentials.password == "password" {
        HttpResponse::Ok().json(LoginPerms {
            sucess: true,
            token: Some("example_token".to_string()),
        })
    } else {
        HttpResponse::Unauthorized().json(LoginPerms {
            sucess: false,
            token: None,
        })
    }
}

#[post("/api/fsub")]
async fn foldercheck(creds: Json<FilePath>) -> HttpResponse {
    //
    println!("{}", &creds.filepath);
    match_logic(&creds.filepath);

    HttpResponse::Unauthorized().json(LoginPerms {
        sucess: false,
        token: None,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("WOW");
    //test_catalog().await;
    HttpServer::new(|| {
        App::new()
            .service(foldercheck)
            .service(logincheck)
            .service(library_send)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
