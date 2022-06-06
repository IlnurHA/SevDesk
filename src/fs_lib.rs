use std::fs;
use std::fs::*;

// Create dir
// Wrapping function to add some checks if it will be necessary
fn create_dir(path: &str) -> Result<(), ()> {
    if metadata(path).is_err() {
        create_dir_all(path);
    }
    Ok(())
}

// Copy file
// Wrapping function
fn copy_file(from_path: &str, to_path: &str) -> Result<(), ()> {
    if let Err(_) = fs::copy(from_path, to_path){
        return Err(());
    }
    Ok(())
}

// List all files in directory
pub fn files_in_dir(path_to_dir: &str) -> Result<Vec<DirEntry>, ()> {
    if let Ok(files) = read_dir(path_to_dir) {
        let mut files_entries = files.map(|x| x.expect("Cannot read file")).collect();
        return Ok(files_entries);
    }
    Err(())
}

// function to join to paths
fn path_join<'a>(path1: &'a str, path2: &'a str) -> String {
    let mut new_path = path1.to_owned();
    new_path += "//";
    new_path += path2;
    return new_path;
}

// function to copy directory and all files and directories inside it
pub fn copy_dir<'a>(from_path: &'a str, to_path: &'a str, dir_name: &'a str) -> Result<(), ()> {
    let mut new_path = path_join(to_path, dir_name);
    if create_dir(new_path.trim()).is_err() {
        return Err(());
    }

    if let Ok(files) = files_in_dir(from_path) {
        for file in files {
            let filename = file.file_name().into_string().expect("Failed to read filename");
            let filetype = file.file_type().expect("Failed to read filetype");
            if filetype.is_file() {
                copy_file(path_join(from_path, filename.trim()).trim(), path_join(new_path.trim(), filename.trim()).trim());
            } else if filetype.is_dir() {
                copy_dir(path_join(from_path, filename.trim()).trim(), new_path.trim(), filename.trim());
            } else { return Err(()); }
        }
    }
    Ok(())
}
