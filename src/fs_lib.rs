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
    let mut new_pathbuf = path_join(to_path, dir_name);
    let mut new_path = new_pathbuf.as_path();
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
                copy_file(
                    path_join(from_path, filename).as_path(),
                    path_join(new_path, filename).as_path(),
                )?;
            } else if filetype.is_dir() {
                copy_dir(
                    path_join(from_path, filename).as_path(),
                    new_path,
                    filename,
                    black_list,
                )?;
            }
        }
    }
    Ok(())
}

pub fn move_all_files(
    from_dir: &Path,
    inside_another_dir: &Path,
    black_list: &[&Path],
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

    println!("{:?}", files);

    for file in files {
        fs_extra::move_items(
            &Vec::from([file.as_path()]),
            inside_another_dir,
            &fs_extra::dir::CopyOptions::new(),
        );
    }

    Ok(())
}
