use std::string;

use actix_web::HttpRequest;
use actix_web::{get, post,web::Json, HttpResponse, App, HttpServer};
use cbztools::catalog_dir;
use matchlogic::match_logic;
use std::path::{Path, PathBuf};
use serde::Serialize;
use serde::Deserialize;
mod cbztools;
mod matchlogic;
#[derive(Deserialize)]
struct FilePath{
    filepath:String,

}


#[derive(Deserialize)]
struct Login{
    username: String,
    password: String,

}


#[derive(Serialize)]
struct LoginPerms{
    sucess: bool,
    token: Option<String>,
}




#[get("/api/library")]
async fn library_send() -> HttpResponse{
    let val = catalog_dir(Path::new("I:\\Comics"));
    println!("{:?}",val);



    HttpResponse::Ok().json(LoginPerms {
        sucess: true,
        token: Some("example_token".to_string())
    })

}




#[post("/api/login")]
async fn logincheck(credentials: Json<Login>) -> HttpResponse {
    if credentials.username == "blarch" && credentials.password == "password" {
        HttpResponse::Ok().json(LoginPerms {
            sucess: true,
            token: Some("example_token".to_string())
        })
    } else {
        HttpResponse::Unauthorized().json(LoginPerms{
            sucess: false,
            token: None
        })
    }
}



#[post("/api/fsub")]
async fn foldercheck(creds: Json<FilePath>) -> HttpResponse { //
    println!("{}",&creds.filepath);
    match_logic(&creds.filepath);

    HttpResponse::Unauthorized().json(LoginPerms{
        sucess: false,
        token: None
    })


}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(foldercheck)
            .service(logincheck)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}