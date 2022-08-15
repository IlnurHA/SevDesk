use fs_extra;
use std::fs;
use std::fs::*;
use std::os::windows::fs::{symlink_dir, symlink_file};
use std::path::{Path, PathBuf};
use std::vec::Vec;

/// Copy file
/// Wrapping function
fn copy_file(from_path: &Path, to_path: &Path) -> std::io::Result<()> {
    fs::copy(
        from_path.to_str().expect("Path is incorrect"),
        to_path.to_str().expect("Path is incorrect"),
    )?;
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
pub fn copy_dir<'a>(
    from_path: &'a Path,
    to_path: &'a Path,
    dir_name: &'a Path,
    black_list: &[&Path],
) -> std::io::Result<()> {
    let new_pathbuf = path_join(to_path, dir_name);
    let new_path = new_pathbuf.as_path();
    create_dir_all(new_path)?;

    if black_list.contains(&new_path) {
        return Ok(());
    }

    if let Ok(files) = files_in_dir(from_path) {
        for file in files {
            let filename_s = file
                .file_name()
                .into_string()
                .expect("Cannot get string of filename");
            let filename = Path::new(filename_s.trim());
            let filetype = file.file_type().expect("Cannot get filetype of file");

            if black_list.contains(&path_join(&from_path, filename).as_path()) {
                continue;
            }

            if filetype.is_file() {
                match copy_file(
                    path_join(from_path, filename).as_path(),
                    path_join(new_path, filename).as_path(),
                ) {
                    Ok(_) => (),
                    Err(message) => println!("{} - {}", filename_s, message),
                };
            } else if filetype.is_dir() {
                match copy_dir(
                    path_join(from_path, filename).as_path(),
                    new_path,
                    filename,
                    black_list,
                ) {
                    Err(message) => println!("{} - {}", filename_s, message),
                    _ => (),
                };
            }
        }
    }
    Ok(())
}

// TODO: move one file
pub fn move_one_file(file_name: String, from_dir: &Path, to_dir: &Path, to_overwrite: bool) {
    let mut file_path_buf = from_dir.join(Path::new(&file_name));
    let mut file_path = file_path_buf.as_path();

    let mut copy_option = fs_extra::dir::CopyOptions::new();
    copy_option.overwrite = to_overwrite;

    fs_extra::move_items(&Vec::from([Path::new(file_path)]), to_dir, &copy_option);
}

pub fn move_all_files(
    from_dir: &Path,
    inside_another_dir: &Path,
    black_list: &[&Path],
    to_overwrite: bool,
) -> std::io::Result<()> {
    create_dir_all(inside_another_dir)?;

    let mut files: Vec<_> = files_in_dir(from_dir)?
        .iter()
        .map(|file| {
            file.file_name()
                .into_string()
                .expect("Cannot get string of filename")
        })
        .filter(|x| !black_list.contains(&Path::new(x)))
        .map(|x| from_dir.join(Path::new(&x)))
        .collect();

    let mut copy_option = fs_extra::dir::CopyOptions::new();
    copy_option.overwrite = to_overwrite;

    println!("{:?}", files);

    for file in files {
        fs_extra::move_items(
            &Vec::from([file.as_path()]),
            inside_another_dir,
            &copy_option,
        );
    }

    Ok(())
}

pub fn create_soft_links(files: &[PathBuf], destination: &Path) -> std::io::Result<()> {
    for file in files {
        if file.as_path().is_file() {
            std::os::windows::fs::symlink_file(
                file.as_path(),
                destination.join(file.as_path()).as_path(),
            )?;
        } else if file.as_path().is_dir() {
            std::os::windows::fs::symlink_dir(
                file.as_path(),
                destination.join(file.as_path()).as_path(),
            )?;
        }
    }
    Ok(())
}

pub fn remove_entity(entity_name: &String, directory: &Path) -> std::io::Result<()> {
    let mut entity_path_buf = directory.join(entity_name);
    let mut entity_path = entity_path_buf.as_path();

    remove_entity_as_path(entity_path)
}

pub fn remove_entity_as_path(entity_path: &Path) -> std::io::Result<()> {
    if entity_path.is_dir() {
        std::fs::remove_dir_all(entity_path)?;
    } else if entity_path.is_file() {
        std::fs::remove_file(entity_path)?;
    }
    Ok(())
}
