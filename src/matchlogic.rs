use std::path::Path;
use sysinfo::Disks;
pub fn match_logic(param: &String) {
    let pac = Path::new(&param);
    println!("=> disks:");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!("{disk:?}");
    }
    //Actual matching logic.
}



#[cfg(target_os = "windows")]
fn diskmatch(){}

#[cfg(target_os = "linux")]
fn diskmatch(){}