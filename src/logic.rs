use crate::fs_lib;
use crate::model::Desktop;
use std::fs::*;
use std::path::Path;
use std::path::PathBuf;

fn files_of(from_dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    Ok(fs_lib::files_in_dir(from_dir)?
        .iter()
        .map(|file| {
            file.file_name()
                .into_string()
                .expect("Cannot get string of filename")
        })
        .map(|x| from_dir.join(Path::new(&x)))
        .collect())
}

// TODO: Expects
pub fn allocate_new_desktop(desktop_name: String, path_of_base: &Path) -> Result<Desktop, String> {
    // if !desktops.into_iter().find(|x| x == desktop_name).is_none() {
    //     return Err("Desktop with this name exists");
    // }

    let mut desktops_path_buf = path_of_base.join(Path::new("desktops"));
    let mut new_desktop_path_buf = desktops_path_buf.as_path().join(Path::new(&desktop_name));
    let mut new_desktop_path = new_desktop_path_buf.as_path();

    if new_desktop_path.is_dir() {
        return Err("Desktop with this name exists".to_owned());
    }

    std::fs::create_dir_all(&new_desktop_path).expect("");
    let mut new_desktop = Desktop::new(desktop_name.clone(), new_desktop_path_buf);

    Ok(new_desktop)
}

pub fn change_desktop(
    current_desktop: &Desktop,
    second_desktop: &Desktop,
    desktop_path: &Path,
    base_path: &Path,
) -> Result<(), String> {
    // Check for existence of current and second desktop
    if !current_desktop.path_buf.is_dir() || !second_desktop.path_buf.is_dir() {
        return Err("No such desktop".to_owned());
    }
    // =============================================
    let top_level_entities_path_buf = base_path.join("topFiles");
    let top_level_entities_path = top_level_entities_path_buf.as_path();

    fs_lib::move_all_files(desktop_path, current_desktop.path_buf.as_path(), &[], true);
    fs_lib::move_all_files(second_desktop.path_buf.as_path(), desktop_path, &[], false);

    // Create link of top level entities
    // fs_lib::create_soft_links()

    let mut files =
        files_of(top_level_entities_path).expect("Failed to read top-level entities from folder");
    fs_lib::create_soft_links(&files, desktop_path)
        .expect("Failed to create soft links of top-level entities");

    Ok(())
}

pub fn first_start<'a>(path_of_desktop: &'a Path, path_of_base: &'a Path) -> Desktop {
    let base_path_for_desktops_buf = path_of_base.join(Path::new("desktops"));
    let base_path_for_desktops = base_path_for_desktops_buf.as_path();
    let blank_path = path_of_base.join(Path::new("blank"));
    let backup_path = path_of_base.join(Path::new("backup"));
    let top_level_files_path = path_of_base.join(Path::new("topFiles"));

    // TODO: expect
    std::fs::create_dir_all(
        base_path_for_desktops
            .join(Path::new("FirstDesktop"))
            .as_path(),
    )
    .expect("");
    std::fs::create_dir_all(&blank_path).expect("");
    std::fs::create_dir_all(&backup_path).expect("");
    std::fs::create_dir_all(&top_level_files_path).expect("");

    match fs_lib::copy_dir(
        path_of_desktop,
        backup_path.as_path(),
        Path::new("first_launch"),
        &[],
    ) {
        Err(message) => println!("{}", message),
        _ => (),
    };

    return Desktop::new("FirstDesktop".to_owned(), path_of_desktop.to_owned());
}

pub fn make_top_level_entity(
    file_name: &String,
    desktop_path: &Path,
    base_path: &Path,
) -> Result<(), String> {
    let mut top_level_path_buf = base_path.join(Path::new("topFiles"));
    let mut top_level_path = top_level_path_buf.as_path();

    let files = files_of(top_level_path).expect("Can't read top-level entities");
    for file in files {
        if let Some(filename) = file.as_path().file_name() {
            if (*filename).to_str().to_owned().unwrap() == file_name {
                return Err("File with this name is existing as top-level entity".to_string());
            }
        }
    }

    let mut file_path_buf = desktop_path.join(Path::new(file_name));
    let mut file_path = file_path_buf.as_path();

    fs_lib::move_one_file(file_name.to_owned(), desktop_path, top_level_path, false);

    let mut links = [file_path_buf];
    fs_lib::create_soft_links(&links, desktop_path);

    Ok(())
}

pub fn remove_top_level_entity(
    file_name: &String,
    desktop_path: &Path,
    base_path: &Path,
) -> Result<(), String> {
    let mut desktop_files = files_of(desktop_path).expect("Cannot read files of current desktop");
    for file in desktop_files {
        if let Some(filename) = file.as_path().file_name() {
            if filename.to_str().unwrap().to_string() == file_name.to_string() {
                return Err(
                    "Entity with such name has already exist in the current desktop".to_string(),
                );
            }
        }
    }

    let mut top_level_path_buf = base_path.join(Path::new("topFiles"));
    let mut top_level_path = top_level_path_buf.as_path();

    fs_lib::remove_entity(file_name, desktop_path);
    fs_lib::move_one_file(file_name.to_string(), top_level_path, desktop_path, false);
    Ok(())
}

// TODO: removal of files and directories of current desktop
pub fn remove_desktop(desktop: &Desktop) -> Result<(), String> {
    let mut files: Vec<_> =
        files_of(desktop.path_buf.as_path()).expect("Can't read files from current desktop");

    for file in files {
        fs_lib::remove_entity_as_path(file.as_path()).expect("Cannot delete file");
    }
    fs_lib::remove_entity_as_path(desktop.path_buf.as_path())
        .expect("Cannot delete desktop folder");
    Ok(())
}
