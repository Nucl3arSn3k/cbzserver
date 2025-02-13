use std::string;

use actix_web::HttpRequest;
use actix_web::{get, post, web::Json, App, HttpResponse, HttpServer};
use cbztools::{cHold, catalog_dir, dbconfig};
use matchlogic::match_logic;
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std::path::{Path, PathBuf};
use rusqlite::{Connection, Result};
mod cbztools;
mod matchlogic;
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
        vax = catalog_dir(Path::new("I:\\Comics"), false).await;

        let e_time = now.elapsed();
        let min_val = e_time.as_secs() / 60;
        let h_val = min_val / 60;

        println!(
            "Scan took {} seconds, which is {} minutes and is {} hours",
            e_time.as_secs(),
            min_val,
            h_val
        );
        let connection = match Connection::open("cache.db") {
            Ok(conn) => conn,
            Err(e) => {
                println!("Error is {}", e);
                return Vec::new();  // Return early from the function with an empty Vec
            }
        };

        for lochold in vax{


            match connection.execute(
                "INSERT INTO files (filepath, name, cover_path, dirornot) VALUES (?1, ?2, ?3, ?4)",
                (
                    &lochold.filepath.to_string_lossy().to_string(),
                    &lochold.name,
                    &lochold.cover_path.as_ref().map(|p| p.to_string_lossy().to_string()),
                    &lochold.dirornot
                ),
            ){
                Ok(_) => (),
                Err(e) => {
                    println!("Error {}",e);
                    
                },
            }


        }
    }
    else{
        //Serialize existing DB to JSON


    }

    match serde_json::to_string_pretty(&vax) {
        Ok(serialized) => println!("Serialized data:\n{}", serialized),
        Err(e) => println!("Serialization error: {}", e),
    }

    HttpResponse::Ok().json(vax)
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
