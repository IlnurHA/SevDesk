mod command_handler;
mod fs_lib;
mod logic;
mod model;
mod tools;

use std::fs::*;
use std::io::{Read, Write};
use std::path::Path;
use std::{fs, io};

use fs_extra::dir::move_dir;

// to handle console commands
fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    buffer
}

fn main() {
    // let path_default = Path::new("C:/rust/default");
    // let path_base = Path::new("C:/rust");
    // let path1 = Path::new("C:/rust/dir1");
    // let path2 = Path::new("C:/rust/dir2");
    //
    // // let path3 = Path::new("C:/rust/dir3");
    //
    // // fs::hard_link(path1, path3).expect("You cannot create hard link for these files");
    //
    // fs_lib::copy_dir(path_default, path_base, Path::new("dir1"), &[]).expect("Copy failed");
    // // fs_lib::move_all_files(path1, path2, &[]).expect("Move failed");
    // logic::first_start(
    //     Path::new("C:/Users/Ilnur/Рабочий стол"),
    //     Path::new("C:/rust"),
    // );
    if !Path::new("C:/rust").is_dir()
        || !Path::new("C:/rust/backup").is_dir()
        || !Path::new("C:/rust/topFiles").is_dir()
        || !Path::new("C:/rust/blank").is_dir()
        || !Path::new("C:/rust/desktops").is_dir()
    {
        println!("Please enter path to the main desktop");
        let desktop_path = read_line();
        logic::first_start(Path::new(&desktop_path), Path::new("C:/rust"));
    }
}
