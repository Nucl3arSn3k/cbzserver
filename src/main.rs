use askama::Template;
use actix_web::{web, post,App, HttpServer, Result,HttpResponse};
use serde::Deserialize;
mod dbhandle;
#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}


#[derive(Template)]
#[template(path="login.html")]
struct LoginTemplate{
    title:String,
}

async fn home_login() -> Result<actix_web::HttpResponse>{
    let template = LoginTemplate{
        title: "Login here".to_string(),

    };
    Ok(actix_web::HttpResponse::Ok().content_type("text/html").body(
        template.render().unwrap()
    ))
}
#[post("/login")]
async fn handle_login(form: web::Form<LoginForm>) -> HttpResponse {
    // Handle login logic here
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(home_login))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
