use std::fs;
use std::fs::*;
use std::path::{Path, PathBuf};

// Create dir
// Wrapping function to add some checks if it will be necessary
fn create_dir(path: &Path) -> Result<(), ()> {
    if metadata(path.to_str().expect("Invalid Unicode")).is_err() {
        create_dir_all(path);
    }
    Ok(())
}

// Copy file
// Wrapping function
fn copy_file(from_path: &Path, to_path: &Path) -> Result<(), ()> {
    if let Err(_) = fs::copy(from_path.to_str().expect("Invalid Unicode"), to_path.to_str().expect("Invalid Unicode")){
        return Err(());
    }
    Ok(())
}

// List all files in directory
pub fn files_in_dir(path_to_dir: &Path) -> Result<Vec<DirEntry>, ()> {
    if let Ok(files) = read_dir(path_to_dir) {
        let mut files_entries = files.map(|x| x.expect("Cannot read file")).collect();
        return Ok(files_entries);
    }
    Err(())
}

// function to join to paths
fn path_join<'a>(path1: &'a Path, path2: &'a Path) -> PathBuf {
    return path1.join(path2);
}

// function to copy directory and all files and directories inside it
pub fn copy_dir<'a>(from_path: &'a Path, to_path: &'a Path, dir_name: &'a Path) -> Result<(), ()> {
    let mut new_pathbuf = path_join(to_path, dir_name);
    let mut new_path = new_pathbuf.as_path();
    if create_dir(new_path).is_err() {
        return Err(());
    }

    if let Ok(files) = files_in_dir(from_path) {
        for file in files {
            let filename_s = file.file_name().into_string().expect("Failed to read filename");
            let filename = Path::new(filename_s.trim());
            let filetype = file.file_type().expect("Failed to read filetype");
            if filetype.is_file() {
                copy_file(path_join(from_path, filename).as_path(), path_join(new_path, filename).as_path());
            } else if filetype.is_dir() {
                copy_dir(path_join(from_path, filename).as_path(), new_path, filename);
            } else { return Err(()); }
        }
    }
    Ok(())
}
