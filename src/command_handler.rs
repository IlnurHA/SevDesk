use crate::fs_lib;
use crate::logic;
use crate::model;
use crate::tools;
use pyo3::prelude::*;
use std::path::PathBuf;
use std::process::Command;

// binds is a vector that contains item of key and value ( binds: [(<bind_name>, <desk_name>)] )
pub struct CommandHandler {
    pub current_desktop: String,
    pub specific_desktops: Vec<model::SpecificDesktop>,
    pub base_path: String,
    pub desktops_path: String,
    pub binds: Vec<(String, String)>,
}

impl CommandHandler {
    pub fn new(desktop_path: String, base_path: String) -> Self {
        let blank_path = PathBuf::from(&base_path).push("blank");
        let current_desktop = "original".to_string();

        // TODO: Add loading of settings

        let mut desktops_path: String = base_path.clone();
        desktops_path.push_str("//");
        desktops_path.push_str("desktops");

        Self {
            current_desktop,
            specific_desktops: vec![logic::first_start(&desktop_path, &base_path)],
            base_path,
            desktops_path,
            binds: vec![],
        }
    }

    pub fn handle(&mut self, command: model::Action) -> Result<(), String> {
        match command {
            model::Action::ChangeDesk { desk_name } => {
                if let Some(specific_desktop) = tools::find(
                    &self.specific_desktops,
                    desk_name.clone(),
                    |x: model::SpecificDesktop| x.name,
                ) {
                    match logic::change_specific_desktop(&specific_desktop) {
                        Ok(_) => {
                            CommandHandler::reboot_explorer();
                            self.current_desktop = desk_name.clone();
                            Ok(())
                        }
                        err => err,
                    }
                } else if tools::find(
                    &logic::files_of(&self.desktops_path).expect("Desktop path is corrupted"),
                    desk_name.clone(),
                    |x: PathBuf| {
                        let string = x
                            .file_name()
                            .expect("Cannot find name of given path")
                            .to_str()
                            .expect("Cannot convert &str from PathBuf")
                            .to_string();
                        println!("{}", string);
                        string
                    },
                )
                .is_some()
                {
                    match logic::change_common_desktop(&desk_name, &self.base_path) {
                        Ok(_) => {
                            CommandHandler::reboot_explorer();
                            self.current_desktop = desk_name.clone();
                            Ok(())
                        }
                        err => err,
                    }
                } else {
                    if desk_name == "blank".to_string() {
                        match self.handle(model::Action::CreateDesk {
                            desk_name: desk_name.clone(),
                        }) {
                            Ok(_) => {
                                match logic::change_common_desktop(&desk_name, &self.base_path) {
                                    Ok(_) => {
                                        CommandHandler::reboot_explorer();
                                        self.current_desktop = desk_name.clone();
                                        Ok(())
                                    }
                                    err => err,
                                }
                            }
                            err => err,
                        }
                    } else {
                        return Err("There is no such desktop".to_string());
                    }
                }
            }
            model::Action::CreateDesk { desk_name } => {
                logic::allocate_new_common_desktop(&desk_name, &self.base_path)
            }
            model::Action::CreateSpecificDesktop { desk_name, path } => {
                // TODO: Add saving specific desktops

                match logic::allocate_new_specific_desktop(
                    &desk_name,
                    &path,
                    &self.specific_desktops,
                ) {
                    Ok(desktop) => {
                        self.specific_desktops.push(desktop);
                        Ok(())
                    }
                    Err(x) => Err(x),
                }
            }
            model::Action::RemoveDesk { desk_name } => {
                // Handling removing of current desktop
                if desk_name == self.current_desktop {
                    match self.handle(model::Action::ChangeDesk {
                        desk_name: "blank".to_string(),
                    }) {
                        Err(message) => return Err(message),
                        Ok(_) => (),
                    }
                }

                // Trying to find desktops from specific desktops
                // and then from common desktops that are stored in base path
                if let Some(specific_desktop) =
                    tools::find(&self.specific_desktops, desk_name.clone(), |x| x.name)
                {
                    return logic::remove_specific_desktop(&specific_desktop);
                } else if tools::find(
                    &logic::files_of(&self.desktops_path).expect("Desktop path is corrupted"),
                    desk_name.clone(),
                    |x| {
                        x.file_name()
                            .expect("desktops path is corrupted")
                            .to_os_string()
                            .into_string()
                            .expect("Cannot make String from OsString")
                    },
                )
                .is_some()
                {
                    return logic::remove_common_desktop(&desk_name, &self.base_path);
                } else {
                    return Err("There is no such desktop".to_string());
                }
            }

            model::Action::CreateBind {
                bind_name,
                desk_name,
            } => {
                if !self.existence_of_desktop_with_name_of(&desk_name) {
                    return Err("Desktop with this name does not exist".to_string());
                }

                if self.existence_of_bind_with_name_of(&bind_name) {
                    return Err("Bind with this name exists".to_string());
                }

                self.binds.push((bind_name, desk_name));
                Ok(())
            }
            model::Action::UseBind { bind_name } => {
                if let Some((_, desk_name)) = tools::find(&self.binds, bind_name, |x| x.0) {
                    return self.handle(model::Action::ChangeDesk { desk_name });
                }
                Err("Bind with this name does not exist".to_string())
            }
            model::Action::RemoveBind { bind_name } => {
                if let Some((_, index)) = tools::find_with_index(&self.binds, bind_name, |x| x.0) {
                    self.binds.swap_remove(index);
                    return Ok(());
                }
                Err("Bind with this name does not exist".to_string())
            }
        }
    }

    fn existence_of_desktop_with_name_of(&self, desk_name: &String) -> bool {
        if tools::find(&self.specific_desktops, desk_name.clone(), |x| x.name).is_some() {
            return true;
        } else if tools::find(
            &logic::files_of(&self.desktops_path).expect("Desktop path is corrupted"),
            desk_name.clone(),
            |x| {
                x.file_name()
                    .expect("desktops path is corrupted")
                    .to_os_string()
                    .into_string()
                    .expect("Cannot make String from OsString")
            },
        )
        .is_some()
        {
            return true;
        }
        false
    }

    fn existence_of_bind_with_name_of(&self, bind_name: &String) -> bool {
        tools::find(&self.binds, bind_name.to_string(), |x| x.0).is_some()
    }

    // TODO: nertsal_commands
    // iter.next() parser
    pub fn parse_command(&mut self, command: String) -> Result<(), String> {
        if command == "" {
            return Err("No command".to_string());
        }
        let mut command_args = command.split_ascii_whitespace();
        match command_args
            .next()
            .expect("There should be at least one word")
        {
            "change_desk" => {
                let mut desk_name_vec = command_args.collect::<Vec<_>>();
                let desk_name = desk_name_vec.join(" ");
                self.handle(model::Action::ChangeDesk { desk_name })
            }
            "create_desk" => self.handle(model::Action::CreateDesk {
                desk_name: command_args.collect::<String>(),
            }),
            "create_specific_desk" => {
                let path = command_args
                    .next()
                    .expect("There should be one more argument");
                let desk_name = command_args.collect::<String>();
                self.handle(model::Action::CreateSpecificDesktop {
                    desk_name,
                    path: path.to_string(),
                })
            }
            "remove_desk" => self.handle(model::Action::RemoveDesk {
                desk_name: command_args.collect::<String>(),
            }),
            "bind" => {
                let bind_name = command_args
                    .next()
                    .expect("There should be one more argument");
                let desk_name = command_args.collect::<String>();
                self.handle(model::Action::CreateBind {
                    bind_name: bind_name.to_string(),
                    desk_name,
                })
            }
            "unbind" => {
                let bind_name = command_args
                    .next()
                    .expect("There should be one more argument");
                self.handle(model::Action::RemoveBind {
                    bind_name: bind_name.to_string(),
                })
            }
            bind_name => self.handle(model::Action::UseBind {
                bind_name: bind_name.to_string(),
            }),
        }
    }

    pub fn read_command(&mut self) {
        println!("Please, enter a command: ");
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer);

        if let Err(message) = self.parse_command(buffer) {
            println!("Failed to make command: \n\t{}", message);
        } else {
            println!("Command successfully handled!");
        }
    }

    pub fn reboot_explorer() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_script = PyModule::from_code(
                py,
                r#"
def reboot_explorer():
    import os

    os.system("taskkill /f /im explorer.exe")
    os.system("start explorer.exe")
    "#,
                "explorer_reboot",
                "explorer_reboot",
            )
            .expect("Cannot read python script");
            py_script
                .getattr("reboot_explorer")
                .expect("Cannot get function 'reboot_explorer' from python script")
                .call0()
                .expect("Cannot call py function");
        });
    }
}
