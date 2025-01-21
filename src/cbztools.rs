
use std::{env, result};
use std::fs::{self, File, ReadDir};
use std::path::{Path, PathBuf};
use tempfile::{tempdir, TempDir};
use unrar::Archive;
use zip::read::ZipArchive;
pub struct cHold{
    name: String,
    filepath:PathBuf,
    dirornot:bool, //true if dir,false if not
} //Shove struct instances into a vec and then shove that to templating engine. In terms of cover display,unrar every single one that's a .cbz and .cbr file and display
//the first one with a image extension as thumb for whatever template
struct templategen{
    name: String,
    filepath:PathBuf,
    cover:PathBuf,

}


#[cfg(target_os = "windows")]
fn get_app_data_dir() -> PathBuf {
    env::var("APPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(r"C:\fallback\path"))
}

#[cfg(target_os = "linux")]
fn get_app_data_dir() -> PathBuf {
    env::var("HOME")
        .map(|home| PathBuf::from(home).join(".config"))
        .unwrap_or_else(|_| PathBuf::from("/fallback/path"))
}

#[cfg(target_os = "macos")]
fn get_app_data_dir() -> PathBuf {
    env::var("HOME")
        .map(|home| PathBuf::from(home).join("Library/Application Support"))
        .unwrap_or_else(|_| PathBuf::from("/fallback/path"))
}

pub fn catalog_dir(dir_path: &Path) -> Vec<cHold> {
    let mut val: Vec<cHold> = Vec::new();
    let result_read = match fs::read_dir(dir_path){
        Ok(entries) => entries,
        Err(_) => return val,
    };

    for entry in result_read{
        let result2 = match entry{
            Ok(direntry) => direntry,
            Err(_) => return val,
        };
        let name_string = result2.file_name().to_string_lossy().to_string();  
        let path = result2.path();
        if path.is_dir(){
            let lochold = cHold{name:name_string,filepath:path.clone(), dirornot:true};
            val.push(lochold);
            let pathv2: &Path = path.as_path();
            catalog_dir(pathv2);
        }
        else{
            let lochold = cHold{name:name_string,filepath:path.clone(), dirornot:false};
            val.push(lochold);



        }


    }

    val
}