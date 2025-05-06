use actix_web::web;
use futures::future::{BoxFuture, FutureExt};

use image::{ExtendedColorType, ImageFormat, ImageReader};
use rusqlite::{Connection, Result};
use serde::Serialize;
use std::fs::{self, File, ReadDir};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, result};
use tempfile::{tempdir, TempDir};
use tokio::fs as tokfs;
use unrar::Archive;
use webp::*;
use zip::read::ZipArchive;
#[derive(Debug, Serialize, Clone)]
pub struct cHold {
    pub name: String,
    pub filepath: PathBuf,
    pub cover_path: Option<PathBuf>,
    pub dirornot: bool, //true if dir,false if not
}

#[derive(Debug, Serialize, Clone)]
pub struct dbHold {
    pub name: String,
    pub filepath: String,
    pub cover_path: Option<String>,
    pub dirornot: i32,
}
//Shove struct instances into a vec and then shove that to templating engine. In terms of cover display,unrar every single one that's a .cbz and .cbr file and display
//the first one with a image extension as thumb for whatever template
struct templategen {
    //Not sure what I was doing with this, I'm going to be honest
    name: String,
    filepath: PathBuf,
    cover: PathBuf,
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

pub fn dir_lister(dir_path: &Path) -> Vec<PathBuf> {
    let mut val: Vec<PathBuf> = Vec::new();
    let result_read = match fs::read_dir(dir_path) {
        Ok(entries) => entries,
        Err(_) => return val,
    };

    for x in result_read {
        if let Ok(entry) = x {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                val.push(entry_path);
            } else {
                continue;
            }
        }
    }

    val
}

pub fn delete_all(dir_path: &PathBuf) -> i32 {
    // just going for C function conventions here,because lazy
    let result_read = match fs::read_dir(dir_path) {
        Ok(entries) => entries,
        Err(_) => return -1,
    };

    for x in result_read.skip(1) {
        let v = x.unwrap();

        let x = fs::remove_file(v.path());

        match x {
            Ok(_) => println!("File removed"),
            Err(_) => return -1,
        }
    }

    0
}
pub fn dbconfig(path: String) -> bool {
    if Path::new(&path).exists() {
        println!("Cache.db exists!");
        return true;
    } else {
        println!("cache.db doesn't exist");
        let connection = match Connection::open("cache.db") {
            Ok(conn) => conn,
            Err(e) => {
                println!("Error is {}", e);
                return false; // Return early from the function
            }
        };

        match connection.execute(
            "CREATE TABLE IF NOT EXISTS files (
                    id INTEGER PRIMARY KEY,
                    filepath TEXT NOT NULL,
                    name TEXT NOT NULL,
                    coverpath TEXT,
                    dirornot INTEGER NOT NULL
                )",
            [],
        ) {
            Ok(_) => false, // Super janky. Could use a custom type to handle this,but don't want to.
            Err(e) => {
                println!("Error creating db {}", e);
                false
            }
        }
    }
}

//Due to the recursion, I think I need a seperate config for the DB
pub async fn catalog_dir(dir_path: &Path, depth: bool) -> Vec<cHold> {
    //Could also generate tree here,will profile preformance later and see what's faster
    let mut val: Vec<cHold> = Vec::new();

    // Now you can use connection here

    let mut read_dir = match tokio::fs::read_dir(dir_path).await {
        Ok(entries) => entries,
        Err(_) => return val,
    };

    let mut futures = Vec::new();

    while let Ok(Some(entry)) = read_dir.next_entry().await {
        let name_string = entry.file_name().to_string_lossy().to_string();
        let path = entry.path();

        let future = async move {
            let mut entries = Vec::new();

            if path.is_dir() {
                let lochold = cHold {
                    name: name_string.clone(),
                    filepath: path.clone(),
                    cover_path: None,
                    dirornot: true,
                };
                entries.push(lochold); //Here's the DB entry

                let mut subdir_entries = catalog_dir(&path, depth).await;
                entries.append(&mut subdir_entries);
            } else {
                if let Some(extension) = path.extension().and_then(std::ffi::OsStr::to_str) {
                    match extension {
                        //JUST CHECK THE MAGIC NUMBERS YOU MORON. "wahh, i don't want to confuse standard rar and zip files". Don't include them
                        "cbz" | "cbr" => {
                            if let Ok(cover_path) = compression_handler(&path, depth).await {
                                let lochold = cHold {
                                    name: name_string,
                                    filepath: path.clone(),
                                    cover_path: Some(cover_path),
                                    dirornot: false,
                                };
                                entries.push(lochold); //Here's where the DB entry should happen
                            } else {
                                println!("Error processing compressed file: {:?}", path);
                            }
                        }
                        _ => {} // Unsupported extension
                    }
                }
            }
            entries
        };

        futures.push(future);
    }

    let results = futures::future::join_all(futures).await;

    for mut result in results {
        val.append(&mut result);
    }

    val
}

pub async fn compression_handler(
    file_path: &Path,
    full_p: bool,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    //Decompresses the file and puts the full version in temp. Modify this to generate a different tag for the covers and to compress the cover images
    let combined_folder_name = if full_p == false {
        file_path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|name| format!("cover-{}", &name[..name.len() - 4]))
            .ok_or("Invalid filename")?
    } else {
        file_path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|name| format!("comictemp-{}", &name[..name.len() - 4]))
            .ok_or("Invalid filename")?
    };

    let temp_dir = get_app_data_dir();
    let temp_dir_path = temp_dir.join(&combined_folder_name);
    fs::create_dir_all(&temp_dir_path)?;

    let is_image = |ext: &str| -> bool {
        ["jpg", "jpeg", "png", "gif", "webp", "bmp"].contains(&ext.to_lowercase().as_str())
    };

    let content = fs::read(file_path)?;
    let slice = &content[..std::cmp::min(content.len(), 7)];

    match slice {
        // RAR signature check
        [0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x00, ..]
        | [0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x01, ..] => {
            println!("Processing RAR file: {:?}", file_path);
            let mut archive = Archive::new(file_path).open_for_processing()?;

            if !full_p {
                // Extract only first image and figure out how to shove in a DB
                while let Some(header) = archive.read_header()? {
                    if header.entry().is_file() {
                        let entry_path = header.entry().filename.to_string_lossy().into_owned();
                        let path = Path::new(&entry_path);
                        if let Some(ext) = path.extension() {
                            if let Some(ext_str) = ext.to_str() {
                                if is_image(ext_str) {
                                    //This is the image (ext_str) convert to webp here
                                    //let img = ImageReader::open(ext_str);

                                    archive = header.extract_with_base(&temp_dir_path)?;
                                    let v = match recursive_file_mover(
                                        &temp_dir_path,
                                        &temp_dir_path,
                                    ) {
                                        Ok(path) => path,
                                        Err(e) => {
                                            println!("File move error {}", e);
                                            PathBuf::new()
                                        }
                                    };
                                    let p2 = v.clone();
                                    let file_name = entry_path.split('/').last().unwrap_or("");
                                    let output_path = temp_dir_path.join(file_name);
                                    println!("Image location is {:?}", v);
                                    let op = output_path.clone(); //Returns output path
                                    let img = match ImageReader::open(v) {
                                        Ok(reader) => match reader.decode() {
                                            Ok(image) => image,
                                            Err(err) => {
                                                eprintln!("Failed to decode image: {}", err);
                                                image::DynamicImage::new_rgb8(1, 1)
                                            }
                                        },
                                        Err(err) => {
                                            eprintln!("Failed to open image: {}", err);
                                            image::DynamicImage::new_rgb8(1, 1) //return a default fail image
                                        }
                                    };
                                    let rgba_img = img.to_rgba8();
                                    let encoderv2 = Encoder::from_rgba(
                                        rgba_img.as_raw(),
                                        rgba_img.width(),
                                        rgba_img.height()
                                    );
                                    
                                    let webp = encoderv2.encode(70.0); // Encode the image

                                    // Extract just the final filename part
                                    let simple_filename = Path::new(file_name)
                                        .file_name() // This gets just the filename portion, without directories
                                        .unwrap_or_default()
                                        .to_string_lossy();

                                    // Now create the webp path with just the filename
                                    let webp_path =
                                        temp_dir_path.join(format!("{}.webp", simple_filename));
                                    std::fs::write(&webp_path, &*webp).unwrap();
                                    println!("Temp dir path is {:?}", temp_dir_path); //simply wipe any file without webp ext
                                    if let Ok(entries) = fs::read_dir(temp_dir_path) {
                                        //use this to iter over dir

                                        for entry in entries {
                                            if let Ok(entry) = entry {
                                                let entry_path = entry.path();
                                                if entry_path
                                                    .extension()
                                                    .map_or(true, |ext| ext != "webp")
                                                {
                                                    if entry_path.is_dir(){

                                                        match fs::remove_dir_all(&entry_path){
                                                            Ok(_) => println!("Successfully removed {:?}",entry_path),
                                                            Err(e) => eprintln!("Failed to remove {:?}: {}",entry_path,e),
                                                        }
                                                    }
                                                    else {
                                                        match fs::remove_file(&entry_path){
                                                            Ok(_) => println!("Successfully removed {:?}", entry_path),
                                                            Err(e) => eprintln!("Failed to remove {:?}: {}",entry_path,e),
                                                        }
                                                        
                                                    }
                                                    
                                                        
                                                }
                                            }
                                        }
                                    }
                                    return Ok(webp_path);
                                }
                            }
                        }
                    }
                    archive = header.skip()?;
                }
                Err("No image found in archive".into())
            } else {
                // Extract everything
                while let Some(header) = archive.read_header()? {
                    archive = if header.entry().is_file() {
                        let entry_path = header.entry().filename.to_string_lossy().into_owned();
                        let file_name = entry_path.split('/').last().unwrap_or("");
                        let output_path = temp_dir_path.join(file_name);
                        header.extract_with_base(&temp_dir_path)?
                    } else {
                        header.skip()?
                    };
                }
                recursive_file_mover(&temp_dir_path, &temp_dir_path);
                Ok(temp_dir_path)
            }
        }
        // ZIP signature check
        [0x50, 0x4B, 0x03, 0x04, ..] => {
            println!("Processing ZIP file: {:?}", file_path);
            let file = File::open(file_path)?;
            let mut archive = ZipArchive::new(file)?;

            if !full_p {
                // Extract only first image
                for i in 0..archive.len() {
                    let mut file = archive.by_index(i)?;
                    //let fc = file.clone();
                    if let Some(ext) = Path::new(file.name()).extension() {
                        if let Some(ext_str) = ext.to_str() {
                            if is_image(ext_str) {
                                let file_name = file.name().split('/').last().unwrap_or("").to_string();
                                let fv2 = file_name.clone();
                                let output_path = temp_dir_path.join(file_name);
                                let mut outfile = File::create(&output_path)?;
                                std::io::copy(&mut file, &mut outfile)?;
                                let img = match ImageReader::open(output_path) {
                                    Ok(reader) => match reader.decode() {
                                        Ok(image) => image,
                                        Err(err) => {
                                            eprintln!("Failed to decode image: {}", err);
                                            image::DynamicImage::new_rgb8(1, 1)
                                        }
                                    },
                                    Err(err) => {
                                        eprintln!("Failed to open image: {}", err);
                                        image::DynamicImage::new_rgb8(1, 1) //return a default fail image
                                    }
                                };
                                let encoder = Encoder::from_image(&img).unwrap();
                                let webp = encoder.encode(70.0); // Encode the image

                                // Extract just the final filename part
                                let simple_filename = Path::new(&fv2)
                                    .file_name() // This gets just the filename portion, without directories
                                    .unwrap_or_default()
                                    .to_string_lossy();

                                // Now create the webp path with just the filename
                                let webp_path =
                                    temp_dir_path.join(format!("{}.webp", simple_filename));
                                std::fs::write(&webp_path, &*webp).unwrap();
                                println!("Temp dir path is {:?}", temp_dir_path); //simply wipe any file without webp ext
                                if let Ok(entries) = fs::read_dir(temp_dir_path) {
                                    //use this to iter over dir

                                    for entry in entries {
                                        if let Ok(entry) = entry {
                                            let entry_path = entry.path();
                                            if entry_path
                                                .extension()
                                                .map_or(true, |ext| ext != "webp")
                                            {
                                                match fs::remove_file(&entry_path){
                                                    Ok(_) => println!("Successfully removed {:?}", entry_path),
                                                    Err(e) => eprintln!("Failed to remove {:?}: {}",entry_path,e),
                                                }
                                                    
                                            }
                                        }
                                    }
                                }
                                return Ok(webp_path);
                            }
                        }
                    }
                }
                Err("No image found in archive".into())
            } else {
                // Extract everything
                archive.extract(&temp_dir_path)?;
                recursive_file_mover(&temp_dir_path, &temp_dir_path);
                Ok(temp_dir_path)
            }
        }
        _ => Err("Unsupported file format".into()),
    }
}

fn recursive_file_mover(
    folder_path: &Path,
    destination_folder: &Path,
) -> Result<PathBuf, std::io::Error> {
    // Make this async
    // if the cbz or cbr file is nested in another folder, this just grabs all the files and puts them in the newly created folder
    if let Ok(entries) = fs::read_dir(folder_path) {
        println!("Recursive file mover triggered");
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    // If it's a dir, recursion happens
                    if let Ok(path) = recursive_file_mover(&entry_path, destination_folder) {
                        return Ok(path);
                    }
                } else {
                    let file_name = entry_path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();

                    // Check if file name is long enough to get extension
                    if file_name.len() < 3 {
                        continue;
                    }

                    // Slice last 3 chars for ext. If it's not (standard set of image)
                    let length = file_name.len();
                    if let Some(ext) = Path::new(&file_name).extension().and_then(|e| e.to_str()) {
                        if !["jpg", "jpeg", "png", "gif", "webp", "bmp"]
                            .contains(&ext.to_lowercase().as_str())
                        {
                            continue;
                        }
                    }
                    let destination_path = destination_folder.join(&file_name); // Fix XML handling
                    match fs::rename(&entry_path, &destination_path) {
                        Ok(_) => {
                            println!("Moved {:?} to {:?}", &entry_path, &destination_path);
                            return Ok(destination_path);
                        }
                        Err(err) => {
                            println!("Failed to move {:?}: {}", &entry_path, err);
                        }
                    }
                }
            }
        }

        // If we got here, no files were moved
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No files were moved",
        ))
    } else {
        // If we couldn't read the folder
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Failed to read folder",
        ))
    }
}
