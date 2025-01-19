
use std::env;
use std::fs::{self, File, ReadDir};
use std::path::{Path, PathBuf};
use tempfile::{tempdir, TempDir};
use unrar::Archive;
use zip::read::ZipArchive;
struct cHold{
    name: String,
    filepath:PathBuf,
    dirornot:bool,



}