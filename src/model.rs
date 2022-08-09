use crate::fs_lib;
use std::path::{Path, PathBuf};

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Desktop {
    pub name: String,
    pub path_buf: PathBuf,
}

impl Desktop {
    pub fn new(name: String, path_buf: PathBuf) -> Self {
        Self { name, path_buf }
    }

    // pub fn swap_desktops(
    //     desktop1: &'a Desktop,
    //     desktop2: &'a Desktop,
    // ) -> (Desktop<'a>, Desktop<'a>) {
    //     let new_desktop1 = Desktop::new(desktop1.name.clone(), desktop2.path);
    //     let new_desktop2 = Desktop::new(desktop2.name.clone(), desktop1.path);
    //     return (new_desktop1, new_desktop2);
    // }
}

enum Action {
    ChangeDesk { desk_name: String },
    CreateDesk { desk_name: String },
    RemoveDesk { desk_name: String },
    MakeTopLevel { entity_name: String },
    MakeNonTopLevel { entity_name: String },
}
