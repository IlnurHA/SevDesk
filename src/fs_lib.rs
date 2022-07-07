use std::fs;
use std::fs::*;
use std::path::{Path, PathBuf};

/// Copy file
/// Wrapping function
fn copy_file(from_path: &Path, to_path: &Path) -> std::io::Result<()> {
    fs::copy(from_path.to_str()?, to_path.to_str()?)?;
    Ok(())
}

/// List all files in directory
pub fn files_in_dir(path_to_dir: &Path) -> std::io::Result<Vec<DirEntry>> {
    read_dir(path_to_dir)?.collect()
}

/// function to join to paths
fn path_join<'a>(path1: &'a Path, path2: &'a Path) -> PathBuf {
    return path1.join(path2);
}

/// function to copy directory and all files and directories inside it
pub fn copy_dir<'a>(from_path: &'a Path, to_path: &'a Path, dir_name: &'a Path, black_list: &[Path]) -> std::io::Result<()> {
    let mut new_pathbuf = path_join(to_path, dir_name);
    let mut new_path = new_pathbuf.as_path();
    create_dir_all(new_path)?;

    if black_list.contains(new_path) {
        return Ok(());
    }

    if let Ok(files) = files_in_dir(from_path) {
        for file in files {
            let filename_s = file.file_name().into_string().expect("Failed to read filename");
            let filename = Path::new(filename_s.trim());
            let filetype = file.file_type().expect("Failed to read filetype");

            if black_list.contains(path_join(from_path, filename)) {
                continue;
            }

            if filetype.is_file() {
                copy_file(path_join(from_path, filename).as_path(), path_join(new_path, filename).as_path());
            } else if filetype.is_dir() {
                copy_dir(path_join(from_path, filename).as_path(), new_path, filename, black_list);
            } else { return Err(()); }
        }
    }
    Ok(())
}
