use std::ptr::metadata;
use std::fs::*;

fn create_dir(path: &str){
    if metadata(path).is_err(){
        create_dir_all(path);
    }
}