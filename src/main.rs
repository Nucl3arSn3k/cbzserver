use actix_web::{post, web, App, HttpResponse, HttpServer, Result};
use askama::Template;
use serde::Deserialize;
use rusqlite::{Connection, Result as otheresult};
mod dbhandle;
#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    title: String,
}

async fn home_login() -> Result<actix_web::HttpResponse> {
    let template = LoginTemplate {
        title: "Login here".to_string(),
    };
    Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap()))
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;


async fn homepage() -> Result<HttpResponse> {
    let template = HomeTemplate {};
    let html = template.render().map_err(|e| {
        actix_web::error::ErrorInternalServerError(e)
    })?;
    
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html))
}
#[post("/login")]
async fn handle_login(form: web::Form<LoginForm>) -> HttpResponse {
    // Handle login logic here,compare users with sqlite DB
    if form.username == "name" && form.password == "blarch" {
        HttpResponse::Found()
            .append_header(("Location", "/homepage"))
            .finish()
    } else {
        HttpResponse::Found()
            .append_header(("Location", "/"))
            .finish()
    }
}
#[actix_web::main]

async fn main() -> std::io::Result<()> {
    let val: otheresult<Connection> = dbhandle::initialize_db();
    match val {
        Ok(v) => println!("DB connection worked{:?}", v),
        Err(e) => println!("DB connection failed {}", e)
    }
    
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(home_login))
            .service(handle_login)
            .service(web::resource("/homepage").to(homepage))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
