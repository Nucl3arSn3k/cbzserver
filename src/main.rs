use std::string;

use actix_web::HttpRequest;
use actix_web::{get, post,web::Json, HttpResponse, App, HttpServer};
use serde::Serialize;
use serde::Deserialize;
mod cbztools;
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
    println!("{}",creds.filepath);


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