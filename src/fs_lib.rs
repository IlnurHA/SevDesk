use std::fs::{read_dir, DirEntry};
use std::path::Path;
use std::vec::Vec;

/// List all files in directory
pub fn files_in_dir(path_to_dir: &Path) -> std::io::Result<Vec<DirEntry>> {
    read_dir(path_to_dir)?.collect()
}

pub fn remove_entity_as_path(entity_path: &Path) -> std::io::Result<()> {
    if entity_path.is_dir() {
        std::fs::remove_dir_all(entity_path)?;
    } else if entity_path.is_file() {
        std::fs::remove_file(entity_path)?;
    }
    Ok(())
}
