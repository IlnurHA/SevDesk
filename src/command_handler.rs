use crate::fs_lib;
use crate::logic;
use crate::model;
use crate::tools;
use std::path::PathBuf;

pub struct CommandHandler {
    pub current_desktop: usize,
    pub desktops: Vec<model::Desktop>,
    pub desktop_path_buf: PathBuf,
    pub base_path_buf: PathBuf,
}

impl CommandHandler {
    pub fn new(
        mut desktops: Vec<model::Desktop>,
        desktop_path_buf: PathBuf,
        base_path_buf: PathBuf,
    ) -> Self {
        let mut blank_path_buf = base_path_buf.as_path().join("blank");
        let mut current_desktop = model::Desktop::new("blank".to_owned(), blank_path_buf);

        desktops.push(current_desktop);
        Self {
            current_desktop: desktops.len() - 1,
            desktops,
            desktop_path_buf,
            base_path_buf,
        }
    }

    pub fn handle(self, command: model::Action) -> Result<(), String> {
        match command {
            model::Action::ChangeDesk => {
                let current_desktop = self.desktops[self.current_desktop];
                if let Some((second_desktop, index)) =
                    tools::find_with_index(self.desktops, command.desk_name, |x| x.name)
                {
                    if let Err(message) = logic::change_desktops(
                        current_desktop,
                        second_desktop,
                        self.desktop_path_buf.as_path(),
                        self.base_path_buf.as_path(),
                    ) {
                        return Err(message);
                    }
                    self.current_desktop = index;
                    Ok(())
                }
                Err("Written desktop does not exist".to_owned())
            }
            model::Action::CreateDesk => {
                match logic::allocate_new_desktop(command.desk_name, self.base_path_buf.as_path()) {
                    Some(desktop) => {
                        self.desktops.push(desktop);
                        Ok(())
                    }
                    Err(message) => Err(message),
                }
            }
            model::Action::RemoveDesk => {
                if let Some(desktop_to_remove, _) =
                    tools::find_with_index(self.desktops, command.desk_name, |x| x.name)
                {
                    if let Err(message) = logic::remove_desktop(desktop_to_remove) {
                        Err(message)
                    }
                    Ok(())
                }
            }
            model::Action::MakeTopLevel => {
                if let Err(message) = logic::make_top_level_entity(
                    command.entity_name,
                    self.desktop_path_buf.as_path(),
                    self.base_path_buf.as_path(),
                ) {
                    Err(message)
                }
                Ok(())
            }
            model::Action::MakeNonTopLevel => {
                if let Err(message) = logic::remove_top_level_entity(
                    command.entity_name,
                    self.desktop_path_buf.as_path(),
                    self.base_path_buf.as_path(),
                ) {
                    Err(message)
                }
                Ok(())
            }
        }
    }

    // TODO: nertsal_commands
    // iter.next() parser
    pub fn parse_command(self, command: String) -> Result<(), String> {
        let command_args = command.split_ascii_whitespace();
        match command_args
            .next()
            .expect("There should be at least one word")
        {
            "change_desk" => self.handle(model::Action::ChangeDesk {
                desk_name: command_args.next().expect("There should be an argument"),
            }),
            "create_desk" => self.handle(model::Action::CreateDesk {
                desk_name: command_args.next().expect("There should be an argument"),
            }),
            "remove_desk" => self.handle(model::Action::RemoveDesk {
                desk_name: command_args.next().expect("There should be an argument"),
            }),
            "to_top" => self.handle(model::Action::MakeTopLevel {
                entity_name: command_args.next().expect("There should be an argument"),
            }),
            "from_top" => self.handle(model::Action::MakeNonTopLevel {
                entity_name: command_args.next().expect("There should be an argument"),
            }),
        }
    }
}
