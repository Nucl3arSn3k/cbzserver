use serde::Serialize;
use std::fs::{self, File, ReadDir};
use std::path::{Path, PathBuf};
use std::{env, result};
use tempfile::{tempdir, TempDir};
use unrar::Archive;
use zip::read::ZipArchive;
#[derive(Debug, Serialize, Clone)]
pub struct cHold {
    name: String,
    filepath: PathBuf,
    cover_path: Option<PathBuf>,
    dirornot: bool, //true if dir,false if not
} //Shove struct instances into a vec and then shove that to templating engine. In terms of cover display,unrar every single one that's a .cbz and .cbr file and display
  //the first one with a image extension as thumb for whatever template
struct templategen {
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

pub fn catalog_dir(dir_path: &Path) -> Vec<cHold> {
    //Should I just provide thumbs here?
    let mut val: Vec<cHold> = Vec::new();
    let result_read = match fs::read_dir(dir_path) {
        Ok(entries) => entries,
        Err(_) => return val,
    };

    for entry in result_read {
        let result2 = match entry {
            Ok(direntry) => direntry,
            Err(_) => return val,
        };
        let name_string = result2.file_name().to_string_lossy().to_string();
        let path = result2.path();
        if path.is_dir() {
            let lochold = cHold {
                name: name_string,
                filepath: path.clone(),
                cover_path: None,
                dirornot: true,
            };
            val.push(lochold);
            let pathv2: &Path = path.as_path();
            let v2al = catalog_dir(pathv2);
            val.extend(v2al);
        } else {
            //Check filetyope here
            if let Some(extension) = path.extension().and_then(std::ffi::OsStr::to_str) {
                match extension {
                    "cbz" => {
                        let va2l = compression_handler(&path, false);
                        let val3 = match va2l {
                            Ok(path_buf) => path_buf,
                            Err(e) => {
                                println!("Error: {}", e);
                                continue; // Use continue instead of break to skip this file
                            }
                        };

                        let lochold = cHold {
                            name: name_string,
                            filepath: path.clone(),
                            cover_path: Some(val3), // Now val3 is PathBuf, not Result<PathBuf>
                            dirornot: false,
                        };
                        val.push(lochold);
                    }

                    "cbr" => {
                        let va2l = compression_handler(&path, false);
                        let val3 = match va2l {
                            Ok(s) => s,
                            Err(e) => {
                                println!("Error: {}", e);
                                break;
                            }
                        };
                        let lochold = cHold {
                            name: name_string,
                            filepath: path.clone(),
                            cover_path: Some(val3.clone()),
                            dirornot: false,
                        };
                        val.push(lochold);
                    }

                    _ => {

                        //Unsupported extension
                    }
                }
            }
        }
    }

    val
}

pub fn compression_handler(
    file_path: &Path,
    full_p: bool,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let combined_folder_name = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .map(|name| format!("comictemp-{}", &name[..name.len() - 4]))
        .ok_or("Invalid filename")?;

    let temp_dir = get_app_data_dir();
    let temp_dir_path = temp_dir.join(&combined_folder_name);
    fs::create_dir_all(&temp_dir_path)?;

    let is_image = |ext: &str| -> bool {
        ["jpg", "jpeg", "png", "gif", "webp", "bmp"]
            .contains(&ext.to_lowercase().as_str())
    };

    let content = fs::read(file_path)?;
    let slice = &content[..std::cmp::min(content.len(), 7)];

    match slice {
        // RAR signature check
        [0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x00, ..] |
        [0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x01, ..] => {
            println!("Processing RAR file: {:?}", file_path);
            let mut archive = Archive::new(file_path).open_for_processing()?;
            
            if !full_p {
                // Extract only first image
                while let Some(header) = archive.read_header()? {
                    if header.entry().is_file() {
                        let entry_path = header.entry().filename.to_string_lossy().into_owned();
                        let path = Path::new(&entry_path);
                        if let Some(ext) = path.extension() {
                            if let Some(ext_str) = ext.to_str() {
                                if is_image(ext_str) {
                                    let file_name = entry_path.split('/').last().unwrap_or("");
                                    let output_path = temp_dir_path.join(file_name);
                                    archive = header.extract_with_base(&temp_dir_path)?;
                                    return Ok(output_path);
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
        },
        // ZIP signature check
        [0x50, 0x4B, 0x03, 0x04, ..] => {
            println!("Processing ZIP file: {:?}", file_path);
            let file = File::open(file_path)?;
            let mut archive = ZipArchive::new(file)?;

            if !full_p {
                // Extract only first image
                for i in 0..archive.len() {
                    let mut file = archive.by_index(i)?;
                    if let Some(ext) = Path::new(file.name()).extension() {
                        if let Some(ext_str) = ext.to_str() {
                            if is_image(ext_str) {
                                let file_name = file.name().split('/').last().unwrap_or("");
                                let output_path = temp_dir_path.join(file_name);
                                let mut outfile = File::create(&output_path)?;
                                std::io::copy(&mut file, &mut outfile)?;
                                return Ok(output_path);
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
        },
        _ => Err("Unsupported file format".into())
    }
}

fn recursive_file_mover(folder_path: &Path, destination_folder: &Path) {
    //if the cbz or cbr file is nested in another folder,this just grabs all the files and puts them in the newly created folder
    if let Ok(entries) = fs::read_dir(folder_path) {
        println!("Recursive file mover triggered");
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    //If it's a dir,recursion happens
                    recursive_file_mover(&entry_path, destination_folder)
                } else {
                    let file_name = entry_path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    //Slice last 3 chars for ext. If it's not (standard set of image )
                    let length = file_name.len();
                    let f_ext = &file_name[length - 3..length];
                    //println!("{}",f_ext);
                    if !["jpg", "jpeg", "png", "gif", "webp", "bmp"].contains(&f_ext) {
                        //Very weird edge case where XML files are there,probably shouldn't extract them.
                        //Will add something later for XML metadata handling
                        continue; //skip to next loop if not a image
                    }
                    let destination_path = destination_folder.join(&file_name); //Fix XML handling
                    if let Err(err) = fs::rename(&entry_path, &destination_path) {
                        println!("Failed to move {:?}: {}", &entry_path, err);
                    } else {
                        println!("Moved {:?} to {:?}", &entry_path, &destination_path);
                    }
                    //println!("Moved {:?} to {:?}", &entry_path, &destination_path);
                }
            }
        }
        //fs::remove_dir(folder_path);
    } else {
        println!("Failed to read folder.");
    }
}
