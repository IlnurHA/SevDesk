extern crate core;

mod command_handler;
mod data_manager;
mod fs_lib;
mod logic;
mod model;
mod regchange;
mod shortcuts;
mod tools;

use crate::data_manager::{
    create_binds_data_file, create_specific_desktops_data_file, is_base_data_file_exist,
    is_binds_data_dile_exist, is_specific_data_file_exist, load_info, read_base,
    write_base_data_file,
};
// use crate::shortcuts;
use crate::shortcuts::{AppManager, KeyBoardState};
use std::path::PathBuf;
use std::{env, io};

// to handle console commands
fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Could not read line");
    buffer.trim().to_string()
}

fn get_env_arguments() -> Option<String> {
    let mut arguments = env::args();
    if arguments.next().is_none() {
        return None;
    }
    return Some(arguments.collect::<Vec<String>>().join(" "));
}

// TODO:
//      convenient command creation
//      restrictions for name of desktops (?)
fn main() {
    if !regchange::is_admin().expect("Cannot get admin privileges") {
        regchange::restart_as_admin(get_env_arguments()).expect("Cannot get admin privileges");
        return;
    }
    clearscreen::clear().expect("Cannot clear screen");

    let mut base_path;

    if !is_base_data_file_exist() {
        loop {
            println!("Enter base path for desktop storage:");
            base_path = read_line();

            if !PathBuf::from(&base_path).is_dir() {
                println!("This dir doesn't exist. Do you want to create it? Y/N");
                let answer = read_line();
                if answer.to_uppercase() == "Y" {
                    std::fs::create_dir_all(PathBuf::from(&base_path)).expect("Cannot create dir");
                    break;
                }
            } else {
                break;
            }
        }
        write_base_data_file(&base_path).expect("Cannot write base path to file");
    } else {
        base_path = read_base().expect("Cannot read base file");
    }

    let path_buf = PathBuf::from(&base_path);

    if !is_specific_data_file_exist(path_buf.as_path()) {
        create_specific_desktops_data_file(path_buf.as_path()).expect("Cannot create file");
    }

    if !is_binds_data_dile_exist(path_buf.as_path()) {
        create_binds_data_file(path_buf.as_path()).expect("Cannot create file");
    }

    let (binds, specific_desktops) = load_info(path_buf.as_path()).expect("Cannot load info");

    println!("System loaded!");
    let mut command_handler =
        command_handler::CommandHandler::new(path_buf, binds, specific_desktops);

    let mut app_manager: AppManager = shortcuts::AppManager::new();

    app_manager.process_parameters(get_env_arguments(), &mut command_handler);
    app_manager.start_program(&mut command_handler, KeyBoardState::new());
}
