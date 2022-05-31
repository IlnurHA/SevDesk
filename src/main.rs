use std::fs::*;
use std::{fs, io};
use std::io::{Read, Write};

fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    buffer
}

fn set_up_default_directories() -> Result<(), ()> {
    if metadata("C://rust").is_err() || metadata("C://rust//default").is_err() {
        create_dir_all("C://rust//default").expect("Cannot create an directory?");
    }
    Ok(())
}

fn copy_files_recursive(path1: &str, path2: &str) {
    for file in fs::read_dir(path1).expect("Hello"){
        println!("{}", file.unwrap().path().display());
    }
}

fn main() {
    // let mut file: File;
    // if metadata("settings.txt").is_err() {
    //     file = File::new("settings.txt");
    //     println!("Please enter the path to the desktop");
    //     file.write_all(read_line().as_bytes());
    // } else {
    //     file = File::open("settings.txt").expect("I cannot open file of settings!");
    // }
    //
    // desktop_path = file.read_to_string().expect("I cannot read the setting file!");
    // set_up_default_directories();
    //
    // println!("Hello, world!");
    copy_files_recursive("C://Users//Ilnur//hs//hw//trafficLights");
}
