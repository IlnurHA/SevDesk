use crate::fs_lib;
use std::path::{Path, PathBuf};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct SpecificDesktop {
    pub name: String,
    pub path: String,
}

impl SpecificDesktop {
    pub fn new(name: String, path: String) -> Self {
        Self { name, path }
    }
}

pub enum Action {
    ChangeDesk {
        desk_name: String,
    },
    CreateDesk {
        desk_name: String,
    },
    RemoveDesk {
        desk_name: String,
    },
    CreateSpecificDesktop {
        desk_name: String,
        path: String,
    },

    CreateBind {
        bind_name: String,
        desk_name: String,
    },
    UseBind {
        bind_name: String,
    },
    RemoveBind {
        bind_name: String,
    },
}
