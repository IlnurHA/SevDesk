mod command_handler;
mod fs_lib;
mod logic;
mod model;
mod regchange;
mod tools;

use pyo3::prelude::*;
use std::io;
use std::io::{Read, Write};

// to handle console commands
fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Could not read line");
    buffer.trim().to_string()
}

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
    Python::with_gil(|py| {
        let py_script = PyModule::from_code(py, py_script_for_elevation, "elevation", "elevation")
            .expect("Cannot read py script");
        py_script
            .getattr("elevation")
            .expect("Cannot get function 'elevation' from python script")
            .call0()
            .expect("Cannot call function");
    });

    println!("Enter current desktop:");
    let desktop_path = read_line();
    println!("Enter base path for desktop storage:");
    let base_path = read_line();

    println!("System loaded!");
    let mut command_handler = command_handler::CommandHandler::new(desktop_path, base_path);
    loop {
        command_handler.read_command();
    }
}
