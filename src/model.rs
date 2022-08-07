use crate::fs_lib;
use std::path::{Path, PathBuf};

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Desktop<'a> {
    pub name: String,
    pub path: &'a Path,
}

impl<'a> Desktop<'a> {
    pub fn new(name: String, path: &'a Path) -> Self {
        Self { name, path }
    }

    pub fn swap_desktops(
        desktop1: &'a Desktop,
        desktop2: &'a Desktop,
    ) -> (Desktop<'a>, Desktop<'a>) {
        let new_desktop1 = Desktop::new(desktop1.name.clone(), desktop2.path);
        let new_desktop2 = Desktop::new(desktop2.name.clone(), desktop1.path);
        return (new_desktop1, new_desktop2);
    }
}
