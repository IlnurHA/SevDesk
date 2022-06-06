mod fs_lib;

use std::fs::*;
use std::{fs, io};
use std::io::{Read, Write};

// to handle console commands
fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    buffer
}

fn main() {
    // testing copy function
    // It's actually working!!!
    let path1 = "C:/Users/user/Desktop";
    let path2 = "C://rust//initial";
    fs_lib::copy_dir(path1, path2, "default");
}
