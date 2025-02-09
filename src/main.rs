use actix_web::middleware::Compress;
use cbztools::{catalog_dir, compression_handler};
use serde_json;
use std::cmp::min;
use std::path::Path;
use std::time::Instant;
mod cbztools;

fn main() {
    let now = Instant::now();
    let val = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(catalog_dir(Path::new("I:\\Comics"), false));
    let e_time = now.elapsed();
    let min_val = e_time.as_secs()/60;
    let h_val = min_val/60;
    println!("Scan took {} seconds, which is {} minutes and is {} hours",e_time.as_secs(),min_val,h_val);
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
