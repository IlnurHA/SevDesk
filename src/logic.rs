use crate::fs_lib;
use crate::model::Desktop;
use std::fs::*;
use std::path::Path;

// TODO: allocate, check for repetitions

// fn check_for_repetitions(desktop_name: String, desktops: Hashmap<String, Desktop>) {}

// pub fn allocate_new_desktop(desktop_name: &, path_of_base: &Path) -> Result<(), String>{
//
// }

pub fn change_desktop(
    current_desktop: &Desktop,
    second_desktop: &Desktop,
    desktop_path: &Path,
) -> Result<(), String> {
    fs_lib::move_all_files(desktop_path, current_desktop.path, &[], true);
    fs_lib::move_all_files(second_desktop.path, desktop_path, &[], false);
    Ok(())
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
            .join(Path::new("CurrentDesktop"))
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
        Ok(_) => (),
        Err(message) => println!("{}", message),
    };

    return Desktop::new("CurrentDesktop".to_owned(), path_of_desktop);
}
