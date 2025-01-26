use std::path::Path;
use actix_web::middleware::Compress;
use serde_json;
use cbztools::{catalog_dir, compression_handler};
mod cbztools;

fn main() {
    
    let val = catalog_dir(Path::new("I:\\Comics"));
    match serde_json::to_string_pretty(&val) {
        Ok(serialized) => println!("Serialized data:\n{}", serialized),
        Err(e) => println!("Serialization error: {}", e),
    }
    /* 
    let x = compression_handler(Path::new("I:\\Comics\\2000AD (0000-2162+)(1977-)\\2000AD 0357 (1984) (Zeg).cbz"), false);

    let pathbuf = match x {
        Ok(p) => p,
        Err(e) => {
            println!("Error: {}", e);
            return; // Or handle error case appropriately
        }
     };

    println!("{:?}",pathbuf);
   */ 
}