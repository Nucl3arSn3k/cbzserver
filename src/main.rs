use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::string;

use actix_web::http::header::ContentType;
use actix_web::{get, post, web::Json, App, HttpResponse, HttpServer};
use actix_web::{web, HttpRequest};
use cbztools::{cHold, catalog_dir, compression_handler, dbconfig};
use image::codecs::webp::WebPEncoder;
use image::{ExtendedColorType, ImageFormat, ImageReader};
use matchlogic::match_logic;
use petgraph::graph::{Graph, NodeIndex};
use rusqlite::{Connection, Result};
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use treegen::{create_graph, dump_graph, FrontendNode, Holder};
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

#[derive(Deserialize)]
struct CoverQuery {
    path: String,
}

struct TreeState {
    global_holder: Mutex<Option<Holder>>, //Hold the graph and the hash
}

#[get("/api/library")]
async fn library_send(data: web::Data<TreeState>) -> HttpResponse {
    let now = std::time::Instant::now();
    let basedir = "I:\\Comics";
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

        //Opens connection to SQLite database
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
    let connection1 = match Connection::open("cache.db") {
        Ok(conn) => conn,
        Err(e) => {
            println!("Error is {}", e);
            return HttpResponse::InternalServerError().body(format!("Error: {}", e));
        }
    };

    let graph = create_graph(connection1);

    //dump_graph(g2.tree);

    if graph.map.contains_key(basedir) {
        let dir_index = graph.map.get(basedir).unwrap();
    }
    let nodeval = FrontendNode::from_graph(&graph, basedir);
    let loc_var;
    if let Some(t_val) = nodeval {
        loc_var = t_val;
    } else {
        println!("No such option");
        return HttpResponse::InternalServerError().body(format!("Error: finding node failed"));
    }

    //loc_var

    match serde_json::to_string_pretty(&loc_var) {
        Ok(serialized) => println!("Serialized data:\n{}", serialized),
        Err(e) => println!("Serialization error: {}", e),
    }

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
    let mut h_tree = data.global_holder.lock().unwrap();
    *h_tree = Some(graph);
    return HttpResponse::Ok().json(loc_var);
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

//Serves the cover
//Serve the issues or fix cover problems?

#[get("/api/cover")]
async fn comic_cover(query: web::Query<CoverQuery>) -> HttpResponse {
    let path = &query.path;
    println!("{:?}", &path);
    
    // Open the file directly and read its bytes
    match std::fs::read(path) {
        Ok(file_bytes) => {
            HttpResponse::Ok()
                .content_type("webp")
                .body(file_bytes)
        },
        Err(err) => {
            eprintln!("Failed to read image file: {}", err);
            HttpResponse::InternalServerError().body(format!("Error: {}", err))
        }
    }
}

#[get("/api/files")]
async fn comic_decompressed(query: web::Query<CoverQuery>) -> HttpResponse {
    //Decompress the file and serve the first img in the decompressed dir to the "viewer"
    let path_st = &query.path;
    let path = Path::new(path_st);
    if let Ok(f_path) = compression_handler(path, true).await {} //Use compression handle

    let temp = "val";
    return HttpResponse::Ok().content_type("image/webp").body(temp);
}

#[get("/api/folders")]
async fn file_open(query: web::Query<CoverQuery>, data: web::Data<TreeState>) -> HttpResponse {
    //Placeholder for file swapping
    let h_tree = data.global_holder.lock().unwrap();
    let path = &query.path;
    let cosref = h_tree.as_ref().unwrap(); //Just unwrap it, we should have something
    let nodeval = FrontendNode::from_graph(cosref, path.as_str());
    let locvar: FrontendNode;
    match nodeval {
        Some(actual) => locvar = actual,
        None => {
            return HttpResponse::InternalServerError().body(format!("Error: finding node failed"));
        } //Cutting off the ex
    }
    let temp = "val";
    return HttpResponse::Ok().json(locvar);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("WOW");
    //let g_state = ;
    //test_catalog().await;
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(TreeState {
                global_holder: Mutex::new(None),
            }))
            .service(foldercheck)
            .service(logincheck)
            .service(library_send)
            .service(comic_cover)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
