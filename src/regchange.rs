use winreg;
use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

pub fn change_desktop_path(new_desktop_path: &String) -> Result<(), String> {
    get_key_for_desktop()?
        .set_value("Desktop", new_desktop_path)
        .map_err(|_| "Cannot change value of regedit subkey")?;
    Ok(())
}
pub fn get_current_desktop_path() -> Result<String, String> {
    let current_desktop = get_key_for_desktop()?
        .get_value("Desktop")
        .map_err(|_| "Cannot reach regedit variable".to_string())?;

    Ok(current_desktop)
}

pub fn add_to_autostart(commands_to_execute: String) -> Result<(), String> {
    let current_process_path = process_path::get_executable_path()
        .ok_or("Cannot get process path".to_string())?
        .display()
        .to_string();

    let arguments = format!("\"{}\" {}", current_process_path, commands_to_execute,);

    get_key_for_autostart()?
        .set_value("SevDesk", &arguments)
        .map_err(|_| "Cannot set value in regedit".to_string())
}

pub fn remove_from_autostart() -> Result<(), String> {
    get_key_for_autostart()?
        .delete_value("SevDesk")
        .map_err(|_| "Cannot remove app from autostart".to_string())
}

pub fn is_in_autostart() -> Result<bool, String> {
    Ok(get_key_for_autostart()?
        .get_value::<String, &str>("SevDesk")
        .is_ok())
}

fn get_key_for_autostart() -> Result<RegKey, String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu
        .create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
        .map_err(|_| "Cannot find path in regedit".to_string())?;
    Ok(key)
}

fn get_key_for_desktop() -> Result<RegKey, String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu
        .create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\User Shell Folders")
        .map_err(|_| "Cannot find path in regedit".to_string())?;
    Ok(key)
}
