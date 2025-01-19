use rocket::{fs::{relative, FileServer}, response::Redirect};
mod cbzlogic;
#[macro_use] extern crate rocket;


#[get("/")]
fn index() -> Redirect{

    Redirect::to("/login.html")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", FileServer::from("static"))
}