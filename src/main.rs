mod command_handler;
mod data_manager;
mod fs_lib;
mod logic;
mod model;
mod regchange;
mod tools;

use crate::data_manager::{
    create_binds_data_file, create_specific_desktops_data_file, is_base_data_file_exist,
    is_binds_data_dile_exist, is_specific_data_file_exist, load_info, read_base,
    write_base_data_file, write_to_bind_data_file,
};
use pyo3::prelude::*;
use std::io;
use std::path::PathBuf;

// to handle console commands
fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Could not read line");
    buffer.trim().to_string()
}

// TODO:
//      'open' and 'help' commands
//      convenient command creation
//      python scripts in separate file
//      restrictions for name of desktops (?)
fn main() {
    pyo3::prepare_freethreaded_python();
    let py_script_for_elevation = r#"
def elevation():
    import ctypes, sys

    def is_admin():
        try:
            return ctypes.windll.shell32.IsUserAnAdmin()
        except:
            return False

    if not is_admin():
        # Re-run the program with admin rights
        ctypes.windll.shell32.ShellExecuteW(None, "runas", sys.executable, " ".join(sys.argv), None, 1)
        exit(0)
        "#;
    let result_python: PyResult<()> = {
        Python::with_gil(|py| -> PyResult<()> {
            let py_script =
                PyModule::from_code(py, py_script_for_elevation, "elevation", "elevation")?;
            py_script.getattr("elevation")?.call0()?;
            Ok(())
        })
        .expect("Don't have admin privileges");
        Ok(())
    };

    if result_python.is_err() {
        println!("Elevation for admin privileges has failed");
        return;
    }

    let mut base_path = String::new();

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
    loop {
        command_handler.read_command();
    }
}
