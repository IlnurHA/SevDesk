use crate::fs_lib;
use crate::model::Desktop;
use std::fs::*;
use std::path::Path;

// TODO: allocate, check for repetitions

// fn check_for_repetitions(desktop_name: String, desktops: Hashmap<String, Desktop>) {}

pub fn allocate_new_desktop(
    desktop_name: String,
    path_of_base: &Path,
    desktops: &Vec<String>,
) -> Result<Desktop, String> {
    if !desktops.into_iter().find(|x| x == desktop_name).is_none() {
        return Err("Desktop with this name exists");
    }

    let mut new_desktop_path_buf = path_of_base.join(Path::new("desktops"));
    let mut new_desktop_path = new_desktop_path_buf.as_path();

    fs_lib::create_dir_all(&new_desktop_path).expect("");

    let mut new_desktop = Desktop::new(desktop_name, new_desktop_path);

    Ok(new_desktop)
}

pub fn change_desktop(
    current_desktop: Desktop,
    second_desktop: Desktop,
    desktop_path: &Path,
    top_level_entities_path: &Path,
    desktops: &Vec<String>,
) -> Result<Desktop, String> {
    // Check for existence of current and second desktop
    if desktops
        .into_iter()
        .find(|x| x == current_desktop.name)
        .is_none()
    {
        return Err("No such desktop");
    }

    if desktops
        .into_iter()
        .find(|x| x == second_desktop.name)
        .is_none()
    {
        return Err("No such desktop");
    }
    // =============================================

    fs_lib::move_all_files(desktop_path, current_desktop.path, &[], true);
    fs_lib::move_all_files(second_desktop.path, desktop_path, &[], false);

    // Create link of top level entities
    // fs_lib::create_soft_links()

    Ok(second_desktop)
}

pub fn first_start<'a>(path_of_desktop: &'a Path, path_of_base: &'a Path) -> Desktop<'a> {
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

    return Desktop::new("FirstDesktop".to_owned(), path_of_desktop);
}

pub fn make_top_level_entity(
    file_name: &String,
    desktop_path: &Path,
    base_path: &Path,
) -> Result<(), String> {
    let mut top_level_path_buf = base_path.join(Path::new("topFiles"));
    let mut top_level_path = top_level_path_buf.as_path();

    let mut file_path_buf = desktop_path.join(Path::new(file_name));
    let mut file_path = file_path_buf.as_path();

    fs_lib::move_file(file_name, desktop_path, top_level_path, false);

    let mut links = [file_path];
    fs_lib::create_soft_links(&links, desktop_path);

    Ok(())
}

pub fn remove_top_level_entity(file_name: &String, desktop_path: &Path, base_path: &Path) {
    let mut top_level_path_buf = base_path.join(Path::new("topFiles"));
    let mut top_level_path = top_level_path_buf.as_path();

    //     TODO: delete soft link

    fs_lib::move_file(file_name, top_level_path, desktop_path);
}

// TODO: removal of files and directories
// pub fn remove_desktop(desktop: &Desktop) {
//
// }
