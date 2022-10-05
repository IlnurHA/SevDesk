use crate::model::SpecificDesktop;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

const SPECIFIC_PATHS_FILE_NAME: &str = "specific_desktops_data_file.txt";
const BASE_PATH_FILE_NAME: &str = "base_data_file.txt";
const BINDS_FILE_NAME: &str = "binds.txt";

/// 1st file stores info about base
/// 2nd file stores info about specific desktops
pub fn write_base_data_file(base_path: &String) -> Result<(), String> {
    let base_data_file_name: String = String::from(BASE_PATH_FILE_NAME);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(base_data_file_name);

    let mut file = file.map_err(|_| String::from("Cannot create file"))?;
    write!(file, "{}", base_path).map_err(|_| String::from("Cannot write to file"))?;

    Ok(())
}

pub fn write_specific_desktop_data_file(
    base_path: &Path,
    specific_desktops: &Vec<SpecificDesktop>,
) -> Result<(), String> {
    let mut new_path = PathBuf::from(base_path);
    new_path.push(SPECIFIC_PATHS_FILE_NAME);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(new_path.as_path());

    let mut file = file.map_err(|_| String::from("Cannot create file"))?;

    for item in specific_desktops {
        write!(file, "{}>>{}\n", item.name, item.path.to_str().unwrap())
            .map_err(|_| String::from("Cannot write to file"))?;
    }
    Ok(())
}

pub fn is_base_data_file_exist() -> bool {
    PathBuf::from(BASE_PATH_FILE_NAME).is_file()
}

pub fn is_specific_data_file_exist(base_path: &Path) -> bool {
    base_path.join(SPECIFIC_PATHS_FILE_NAME).is_file()
}

pub fn is_binds_data_dile_exist(base_path: &Path) -> bool {
    base_path.join(BINDS_FILE_NAME).is_file()
}

pub fn read_base() -> Result<String, String> {
    let base_path = PathBuf::from(BASE_PATH_FILE_NAME);
    let base_path = fs::read_to_string(base_path).map_err(|_| String::from("Cannot read file"))?;
    Ok(base_path)
}

pub fn read_specific_desktops(base_path: &Path) -> Result<Vec<SpecificDesktop>, String> {
    let specific_desktops_paths_file = base_path.join(SPECIFIC_PATHS_FILE_NAME);

    let file = OpenOptions::new()
        .read(true)
        .open(specific_desktops_paths_file)
        .map_err(|_| String::from("Cannot open a file"))?;

    let reader = BufReader::new(file);
    Ok(get_specific_desktops(reader))
}

fn get_specific_desktops(reader: BufReader<File>) -> Vec<SpecificDesktop> {
    let mut specific_desktops: Vec<SpecificDesktop> = vec![];

    for line in reader.lines() {
        if let Ok(arguments) = line {
            let mut arguments = arguments.split(">>");
            let name = arguments.next();

            if name.is_none() {
                break;
            }

            specific_desktops.push(SpecificDesktop::new(
                String::from(name.unwrap()),
                PathBuf::from(arguments.collect::<Vec<&str>>().join(" ")),
            ));
        }
    }

    specific_desktops
}

pub fn write_to_bind_data_file(
    binds: &Vec<(String, String)>,
    base_path: &Path,
) -> Result<(), String> {
    let binds_path = base_path.join(BINDS_FILE_NAME);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(binds_path)
        .map_err(|_| String::from("Cannot open file"))?;

    for (bind_name, desk_name) in binds {
        write!(file, "{} {}\n", bind_name, desk_name)
            .map_err(|_| String::from("Cannot write to binds file"))?;
    }

    Ok(())
}

pub fn read_binds(base_path: &Path) -> Result<Vec<(String, String)>, String> {
    let path = base_path.join(BINDS_FILE_NAME);

    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(|_| String::from("Cannot open file to read binds"))?;

    let reader = BufReader::new(file);

    Ok(get_binds(reader))
}

fn get_binds(reader: BufReader<File>) -> Vec<(String, String)> {
    let mut binds: Vec<(String, String)> = vec![];

    for line in reader.lines() {
        if let Ok(arguments) = line {
            let mut arguments = arguments.split_whitespace();
            let bind_name = arguments.next();

            if bind_name.is_none() {
                break;
            }

            binds.push((String::from(bind_name.unwrap()), arguments.collect()));
        }
    }

    binds
}

pub fn create_binds_data_file(base_path: &Path) -> Result<(), String> {
    let path = base_path.join(BINDS_FILE_NAME);

    create_file(path.as_path())
}

pub fn create_base_data_file() -> Result<(), String> {
    let path = PathBuf::from(BASE_PATH_FILE_NAME);
    create_file(path.as_path())
}

pub fn create_specific_desktops_data_file(base_path: &Path) -> Result<(), String> {
    let path = base_path.join(SPECIFIC_PATHS_FILE_NAME);
    create_file(path.as_path())
}

fn create_file(path: &Path) -> Result<(), String> {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|_| String::from("Cannot create file"))?;
    Ok(())
}

pub fn load_info(
    base_path: &Path,
) -> Result<(Vec<(String, String)>, Vec<SpecificDesktop>), String> {
    Ok((read_binds(base_path)?, read_specific_desktops(base_path)?))
}
