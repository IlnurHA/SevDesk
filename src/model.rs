use std::path::PathBuf;

pub const SPECIFIC_PATHS_FILE_NAME: &str = "specific_desktops_data_file.txt";
pub const BASE_PATH_FILE_NAME: &str = "base_data_file.txt";
pub const BINDS_FILE_NAME: &str = "binds.txt";
pub const DESKTOP_DIR_NAME: &str = "desktops";

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct SpecificDesktop {
    pub name: String,
    pub path: PathBuf,
}

impl SpecificDesktop {
    pub fn new(name: String, path: PathBuf) -> Self {
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
        path: PathBuf,
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

    DeskList,
    CommandsList,
    BindsList,
    ClearCommandLine,
    OpenInExplorer {
        desk_name: String,
    },
    AddToAutoStart {
        commands: String, // Commands separated by ';'
    },
    RemoveFromAutoStart,
}
