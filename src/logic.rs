use crate::fs_lib;
use crate::model::SpecificDesktop;
use crate::regchange;
use crate::tools;
use std::path::Path;
use std::path::PathBuf;

pub fn files_of(from_dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let path: PathBuf = from_dir.to_owned();
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

// TODO: Expects
pub fn allocate_new_common_desktop(
    desktop_name: &String,
    path_of_base: &Path,
) -> Result<(), String> {
    let mut desktops_path_buf: PathBuf = path_of_base.to_owned();
    desktops_path_buf.push("desktops");
    let new_desktop_path_buf = desktops_path_buf.as_path().join(Path::new(desktop_name));
    let new_desktop_path = new_desktop_path_buf.as_path();

    if new_desktop_path.is_dir() {
        return Err("Desktop with this name exists".to_string());
    }

    std::fs::create_dir_all(&new_desktop_path)
        .map_err(|_| "Cannot create directory for new desktop".to_string())?;

    Ok(())
}

pub fn allocate_new_specific_desktop(
    desktop_name: &String,
    desktop_path: &Path,
    specific_desktops: &Vec<SpecificDesktop>,
) -> Result<SpecificDesktop, String> {
    if tools::find_with_index(specific_desktops, desktop_name.clone(), |x| x.name).is_some() {
        return Err("Desktop with this name exists".to_string());
    }

    Ok(SpecificDesktop::new(
        desktop_name.to_string(),
        desktop_path.to_owned(),
    ))
}

pub fn change_common_desktop(desktop: &String, path_of_base: &Path) -> Result<(), String> {
    // Check for existence of current and second desktop
    let mut path: PathBuf = path_of_base.to_owned();
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
    regchange::change_desktop_path(&path)?;
    Ok(())
}

pub fn remove_common_desktop(desktop_name: &String, path_of_base: &Path) -> Result<(), String> {
    let mut desktop_path: PathBuf = path_of_base.to_owned();
    desktop_path.push("desktops");
    desktop_path.push(desktop_name);

    base_remove_desktop(&desktop_path)
}

fn base_remove_desktop(desktop_path: &Path) -> Result<(), String> {
    let files: Vec<_> = files_of(desktop_path).expect("Can't read files from current desktop");

    for file in files {
        if fs_lib::remove_entity_as_path(file.as_path()).is_err() {
            return Err(format!(
                "Cannot delete file {}",
                file.to_str().expect("Cannot convert &str from PathBuf")
            ));
        }
    }
    if fs_lib::remove_entity_as_path(Path::new(desktop_path)).is_err() {
        return Err(format!("Cannot delete desktop {}", desktop_path.display()));
    }
    Ok(())
}

pub fn get_common_desktop(desktops_path: &Path, desk_name: String) -> Option<PathBuf> {
    tools::find(
        &files_of(desktops_path).expect("Desktop path is corrupted"),
        desk_name.clone(),
        |x| {
            x.file_name()
                .expect("desktops path is corrupted")
                .to_os_string()
                .into_string()
                .expect("Cannot make String from OsString")
        },
    )
}

pub fn get_specific_desktop(
    desktops: &Vec<SpecificDesktop>,
    desk_name: String,
) -> Option<SpecificDesktop> {
    tools::find(&desktops, desk_name.clone(), |x: SpecificDesktop| x.name)
}
pub fn get_current_desktop() -> Result<(String, PathBuf), String> {
    let path = PathBuf::from(regchange::get_current_desktop_path()?);

    let name = String::from(
        path.file_name()
            .expect("Cannot read current desktop name")
            .to_str()
            .expect("Cannot read current desktop name"),
    );

    Ok((name, path))
}
