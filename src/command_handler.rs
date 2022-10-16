use crate::data_manager::{write_specific_desktop_data_file, write_to_bind_data_file};
use crate::logic;
use crate::logic::{get_common_desktop, get_specific_desktop};
use crate::model;
use crate::model::SpecificDesktop;
use crate::tools;
use std::path::PathBuf;
use std::process::Command;

// binds is a vector that contains item of key and value ( binds: [(<bind_name>, <desk_name>)] )
pub struct CommandHandler {
    pub specific_desktops: Vec<model::SpecificDesktop>,
    pub base_path: PathBuf,
    pub desktops_path: PathBuf,
    pub binds: Vec<(String, String)>,
}

impl CommandHandler {
    pub fn new(
        base_path: PathBuf,
        binds: Vec<(String, String)>,
        specific_desktops: Vec<SpecificDesktop>,
    ) -> Self {
        let mut desktops_path: PathBuf = base_path.clone();
        desktops_path.push("desktops");

        Self {
            specific_desktops,
            base_path,
            desktops_path,
            binds,
        }
    }

    pub fn handle(&mut self, command: model::Action) -> Result<(), String> {
        match command {
            model::Action::ChangeDesk { desk_name } => {
                if let Some(specific_desktop) =
                    get_specific_desktop(&self.specific_desktops, desk_name.clone())
                {
                    logic::change_specific_desktop(&specific_desktop)?;
                    CommandHandler::reboot_explorer()?;
                    Ok(())
                } else if get_common_desktop(self.desktops_path.as_path(), desk_name.clone())
                    .is_some()
                {
                    logic::change_common_desktop(&desk_name, &self.base_path)?;
                    CommandHandler::reboot_explorer()?;
                    Ok(())
                } else {
                    if desk_name == String::from("blank") {
                        self.handle(model::Action::CreateDesk {
                            desk_name: desk_name.clone(),
                        })?;
                        logic::change_common_desktop(&desk_name, &self.base_path)?;
                        CommandHandler::reboot_explorer()?;
                        Ok(())
                    } else {
                        return Err("There is no such desktop".to_string());
                    }
                }
            }
            model::Action::CreateDesk { desk_name } => {
                logic::allocate_new_common_desktop(&desk_name, &self.base_path)
            }
            model::Action::CreateSpecificDesktop { desk_name, path } => {
                if !path.is_dir() {
                    return Err(String::from("No such directory"));
                }

                let desktop = logic::allocate_new_specific_desktop(
                    &desk_name,
                    &path,
                    &self.specific_desktops,
                )?;
                self.specific_desktops.push(desktop);
                write_specific_desktop_data_file(
                    self.base_path.as_path(),
                    &self.specific_desktops,
                )?;
                Ok(())
            }
            model::Action::RemoveDesk { desk_name } => {
                // TODO: Handling removing of current desktop
                if desk_name == logic::get_current_desktop()?.0 {
                    self.handle(model::Action::ChangeDesk {
                        desk_name: String::from("blank"),
                    })?;
                }

                // Trying to find desktops from specific desktops
                // and then from common desktops that are stored in base path
                return if let Some((_, index)) =
                    tools::find_with_index(&self.specific_desktops, desk_name.clone(), |x| x.name)
                {
                    self.specific_desktops.swap_remove(index);
                    Ok(())
                } else if logic::get_common_desktop(self.desktops_path.as_path(), desk_name.clone())
                    .is_some()
                {
                    logic::remove_common_desktop(&desk_name, self.base_path.as_path())
                } else {
                    Err("There is no such desktop".to_string())
                };
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
                write_to_bind_data_file(&self.binds, self.base_path.as_path())?;
                Ok(())
            }
            model::Action::UseBind { bind_name } => {
                if let Some((_, desk_name)) = tools::find(&self.binds, bind_name, |x| x.0) {
                    return self.handle(model::Action::ChangeDesk { desk_name });
                }
                Err(String::from("Bind with this name does not exist"))
            }
            model::Action::RemoveBind { bind_name } => {
                if let Some((_, index)) = tools::find_with_index(&self.binds, bind_name, |x| x.0) {
                    self.binds.swap_remove(index);
                    write_to_bind_data_file(&self.binds, self.base_path.as_path())?;
                    return Ok(());
                }
                Err(String::from("Bind with this name does not exist"))
            }
            model::Action::DeskList => {
                println!("Common desktops:");

                for desk in logic::files_of(self.desktops_path.as_path())
                    .map_err(|_| String::from("Cannot read files"))?
                {
                    if desk.is_dir() {
                        println!(
                            "\t{}",
                            desk.file_name()
                                .ok_or(String::from("Cannot read file name"))?
                                .to_str()
                                .ok_or(String::from("Cannot convert to str"))?
                        );
                    }
                }

                println!("Specific desktops:");
                for SpecificDesktop { name, path } in &self.specific_desktops {
                    println!(
                        "\t{} - {}",
                        name,
                        path.to_str()
                            .ok_or(String::from("Cannot convert to string"))?
                    );
                }

                Ok(())
            }
            model::Action::BindsList => {
                if self.binds.is_empty() {
                    println!("No binds");
                    return Ok(());
                }

                println!("Binds:");
                for (bind_name, desk_name) in &self.binds {
                    println!("\t{} - {}", bind_name, desk_name);
                }

                Ok(())
            }
            model::Action::ClearCommandLine => {
                clearscreen::clear().map_err(|_| "Cannot clear screen".to_string())?;
                Ok(())
            }
            model::Action::OpenInExplorer { desk_name } => {
                let openning_desk;

                if logic::get_common_desktop(self.desktops_path.as_path(), desk_name.clone())
                    .is_some()
                {
                    openning_desk = self.desktops_path.join(desk_name).display().to_string();
                } else if let Some(desk_path) =
                    logic::get_specific_desktop(&self.specific_desktops, desk_name.clone())
                {
                    openning_desk = desk_path.path.display().to_string();
                } else {
                    return Err("No such desktop".to_string());
                }

                Command::new("explorer")
                    .arg(openning_desk)
                    .spawn()
                    .map_err(|_| "Cannot open file through explorer".to_string())?;
                Ok(())
            }
            // Hardcoded for now
            model::Action::CommandsList => {
                println!("Commands:");
                println!("\tcreate_desk (or cd) <desk_name> \tcreate common desk in base with <desk_name>");
                println!("\tcreate_specific_desk <desk_name> <path> \tcreate specific desktop with <desk_name> and <path>");
                println!("\tremove_desk <desk_name> \tremoves desk and files if it is a common desk and removes specific desk name from system");
                println!("\tbind <bind_name> <desk_name> \tcreates <bind_name> for desk with <desk_name>");
                println!("\tunbind <bind_name> \tdeletes bind from system");
                println!("\tdesk_list (or dl) \tprints all desktops");
                println!("\tbinds_list (or bl) \tprints all binds");
                println!("\thelp \tprints all commands");
                println!("\tclear \tclearing command line");
                println!("\topen <desk_name> \topens desktop directory in explorer");
                println!("\t<bind_name> \ttry to switch to desk with bind <bind_name>");

                println!("If command doesn't match to any command above, it will try to use bind");
                println!("For now, it is possible to bind with name as command");

                println!(
                    "This is hardcoded help. So, if something is missing, you can make an issue"
                );

                Ok(())
            }
        }
    }

    fn existence_of_desktop_with_name_of(&self, desk_name: &String) -> bool {
        if logic::get_specific_desktop(&self.specific_desktops, desk_name.clone()).is_some() {
            return true;
        } else if logic::get_common_desktop(self.desktops_path.as_path(), desk_name.clone())
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
            return Err(String::from("No command"));
        }
        let mut command_args = command.split_ascii_whitespace();
        match command_args
            .next()
            .ok_or(String::from("There should be at least one word"))?
        {
            "change_desk" | "cd" => {
                let desk_name = String::from(
                    command_args
                        .next()
                        .ok_or(String::from("There should be one more argument"))?,
                );

                if command_args.next().is_some() {
                    return Err(String::from("Too many arguments"));
                }

                self.handle(model::Action::ChangeDesk { desk_name })
            }
            "create_desk" => {
                let desk_name = String::from(
                    command_args
                        .next()
                        .ok_or(String::from("Too few arguments"))?,
                );

                if command_args.next().is_some() {
                    return Err(String::from("Too many arguments"));
                }

                self.handle(model::Action::CreateDesk { desk_name })
            }
            "create_specific_desk" => {
                let desk_name = command_args
                    .next()
                    .ok_or(String::from("Too few arguments"))?
                    .to_string();
                let path = PathBuf::from(command_args.collect::<Vec<_>>().join(" "));

                self.handle(model::Action::CreateSpecificDesktop { desk_name, path })
            }
            "remove_desk" => {
                let desk_name = String::from(
                    command_args
                        .next()
                        .ok_or(String::from("Too few arguments"))?,
                );

                if command_args.next().is_some() {
                    return Err(String::from("Too many arguments"));
                }

                self.handle(model::Action::RemoveDesk { desk_name })
            }
            "bind" => {
                let bind_name = String::from(
                    command_args
                        .next()
                        .ok_or(String::from("Too few arguments"))?,
                );
                let desk_name = String::from(
                    command_args
                        .next()
                        .ok_or(String::from("Too few arguments"))?,
                );

                if command_args.next().is_some() {
                    return Err(String::from("Too many arguments"));
                }

                self.handle(model::Action::CreateBind {
                    bind_name,
                    desk_name,
                })
            }
            "unbind" => {
                let bind_name = String::from(command_args.next().ok_or("Too few arguments")?);

                if command_args.next().is_some() {
                    return Err(String::from("Too many arguments"));
                }

                self.handle(model::Action::RemoveBind { bind_name })
            }
            "desk_list" | "dl" => self.handle(model::Action::DeskList),
            "bind_list" | "bl" => self.handle(model::Action::BindsList),
            "clear" => self.handle(model::Action::ClearCommandLine),
            "help" => self.handle(model::Action::CommandsList),
            "open" => {
                let desk_name = String::from(command_args.next().ok_or("Too few arguments")?);

                if command_args.next().is_some() {
                    return Err(String::from("Too many arguments"));
                }

                self.handle(model::Action::OpenInExplorer { desk_name })
            }
            bind_name => self.handle(model::Action::UseBind {
                bind_name: String::from(bind_name),
            }),
        }
    }

    pub fn read_command(&mut self) {
        println!("Please, enter a command: ");
        let mut buffer = String::new();
        if let Err(message) = std::io::stdin().read_line(&mut buffer) {
            println!("Failed to read from stdin: \n\t{}", message);
        }

        if let Err(message) = self.parse_command(buffer) {
            println!("Failed to parse command: \n\t{}", message);
        } else {
            println!("Command successfully handled!");
        }
    }

    pub fn reboot_explorer() -> Result<(), String> {
        Command::new("cmd")
            .args(["/C", "taskkill", "/f", "/im", "explorer.exe"])
            .spawn()
            .map_err(|_| "Cannot kill explorer.exe".to_string())?
            .wait()
            .map_err(|_| "Cannot kill explorer".to_string())?;

        Command::new("cmd")
            .arg("/C")
            .arg("start")
            .arg("explorer.exe")
            .spawn()
            .map_err(|_| "Cannot start explorer.exe".to_string())?
            .wait()
            .map_err(|_| "Cannot start explorer.exe".to_string())?;

        Ok(())
    }
}
