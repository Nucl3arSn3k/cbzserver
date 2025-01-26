use std::path::Path;
use serde_json;
use cbztools::catalog_dir;
mod cbztools;

fn main() {
    let val = catalog_dir(Path::new("I:\\Comics"));
    match serde_json::to_string_pretty(&val) {
        Ok(serialized) => println!("Serialized data:\n{}", serialized),
        Err(e) => println!("Serialization error: {}", e),
    }
}