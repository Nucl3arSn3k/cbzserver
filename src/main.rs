use rocket::{
    form::Form,  // Changed this import
    fs::{relative, FileServer},
    response::Redirect,
};
use std::path::{Path, PathBuf};
mod cbzlogic;

#[macro_use]
extern crate rocket;

#[derive(FromForm)]
struct FolderPath {
    folder_path: String,
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/login.html")
}

// Added data parameter
#[post("/submit_folder", data = "<form>")]
fn submit_folder(form: Form<FolderPath>) -> String {
    println!("Folder path is {}", form.folder_path);  // Use form.folder_path instead of form.value
    form.folder_path.clone()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, submit_folder])  // Added submit_folder to routes
        .mount("/", FileServer::from("static"))
}