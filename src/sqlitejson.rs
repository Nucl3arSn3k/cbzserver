use crate::cbztools::dbHold;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use std::error::Error;
use anyhow::Result;


pub fn sq_to_json_boxed(con: Connection) -> Result<String> { //Cleaned this up with anyhow,didn't want to write an error wrapper
    let mut indiv = con.prepare("SELECT name, filepath, coverpath, dirornot FROM files")?;
    
    let entry_iter = indiv.query_map([], |row| {
        Ok(dbHold {
            name: row.get(0)?,
            filepath: row.get(1)?,
            cover_path: row.get(2)?,
            dirornot: row.get(3)?,
        })
    })?; //Convert into a struct. May want to define that here actually,little unclear as of now
    
    let mut objects = Vec::new();
    for entry in entry_iter {
        objects.push(entry?);  
    }
    
    let result = serde_json::json!({ "items": objects });
    Ok(serde_json::to_string(&result)?)
}
