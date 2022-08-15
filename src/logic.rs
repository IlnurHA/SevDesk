use crate::fs_lib;
use crate::model::SpecificDesktop;
use crate::regchange;
use crate::tools;
use std::fs::*;
use std::path::Path;
use std::path::PathBuf;

pub fn files_of(from_dir: &String) -> std::io::Result<Vec<PathBuf>> {
    let path = PathBuf::from(from_dir);
    Ok(fs_lib::files_in_dir(path.as_path())?
        .iter()
        .map(|file| {
            file.file_name()
                .into_string()
                .expect("Cannot get string of filename")
        })
        .map(|x| path.join(Path::new(&x)))
        .collect())
}

pub fn first_start(path_of_desktop: &String, path_of_base: &String) -> SpecificDesktop {
    let path_of_base_buf = PathBuf::from(path_of_base);
    let base_path_for_desktops_buf = path_of_base_buf.join(Path::new("desktops"));
    let base_path_for_desktops = base_path_for_desktops_buf.as_path();

    // TODO: expect
    std::fs::create_dir_all(base_path_for_desktops).expect("");

    SpecificDesktop::new("original".to_string(), path_of_desktop.to_string())
}

// TODO: Expects
pub fn allocate_new_common_desktop(
    desktop_name: &String,
    path_of_base: &String,
) -> Result<(), String> {
    let mut desktops_path_buf = PathBuf::from(path_of_base);
    desktops_path_buf.push("desktops");
    let mut new_desktop_path_buf = desktops_path_buf.as_path().join(Path::new(desktop_name));
    let mut new_desktop_path = new_desktop_path_buf.as_path();

    if new_desktop_path.is_dir() {
        return Err("Desktop with this name exists".to_owned());
    }

    std::fs::create_dir_all(&new_desktop_path).expect("");

    Ok(())
}

pub fn allocate_new_specific_desktop(
    desktop_name: &String,
    desktop_path: &String,
    specific_desktops: &Vec<SpecificDesktop>,
) -> Result<SpecificDesktop, String> {
    if tools::find_with_index(specific_desktops, desktop_name.clone(), |x| x.name).is_some() {
        return Err("Desktop with this name exists".to_string());
    }

    Ok(SpecificDesktop::new(
        desktop_name.to_string(),
        desktop_path.to_string(),
    ))
}

pub fn change_common_desktop(desktop: &String, path_of_base: &String) -> Result<(), String> {
    // Check for existence of current and second desktop
    let mut path = PathBuf::from(path_of_base);
    path.push("desktops");
    path.push(desktop);
    base_change_desktop(path)
}

pub fn change_specific_desktop(desktop: &SpecificDesktop) -> Result<(), String> {
    base_change_desktop(PathBuf::from(desktop.path.clone()))
}

fn base_change_desktop(path_buf: PathBuf) -> Result<(), String> {
    if !path_buf.is_dir() {
        return Err("No such desktop".to_owned());
    }

    let path = path_buf.to_str().expect("").to_string();
    regchange::change_desktop_path(&path);
    Ok(())
}

// TODO: removal of files and directories of current desktop
pub fn remove_specific_desktop(desktop: &SpecificDesktop) -> Result<(), String> {
    base_remove_desktop(&desktop.path)
}

pub fn remove_common_desktop(desktop_name: &String, path_of_base: &String) -> Result<(), String> {
    let mut desktop_path = path_of_base.to_string().clone();
    desktop_path.push_str("//desktops//");
    desktop_path.push_str(desktop_name);

    base_remove_desktop(&desktop_path)
}

fn base_remove_desktop(desktop_path: &String) -> Result<(), String> {
    println!("{}", desktop_path);
    let mut files: Vec<_> = files_of(desktop_path).expect("Can't read files from current desktop");

    for file in files {
        if fs_lib::remove_entity_as_path(file.as_path()).is_err() {
            return Err(format!(
                "Cannot delete file {}",
                file.to_str().expect("Cannot convert &str from PathBuf")
            ));
        }
    }
    if fs_lib::remove_entity_as_path(Path::new(desktop_path)).is_err() {
        return Err(format!("Cannot delete desktop {}", desktop_path));
    }
    Ok(())
}
