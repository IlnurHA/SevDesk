use winreg;
use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;
use winsafe::prelude::*;
use winsafe::HWND;

pub fn change_desktop_path(new_desktop_path: &String) -> Result<(), String> {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hklm
        .create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\User Shell Folders")
        .map_err(|_| "Cannot find path in regedit".to_string())?;
    key.set_value("Desktop", new_desktop_path)
        .map_err(|_| "Cannot change value of regedit subkey")?;

    drop(key);
    Ok(())
}
pub fn get_current_desktop_path() -> Result<String, String> {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hklm
        .create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\User Shell Folders")
        .map_err(|_| "Cannot find path in regedit".to_string())?;

    let current_desktop = key
        .get_value("Desktop")
        .map_err(|_| "Cannot reach regedit variable".to_string())?;
    drop(key);

    Ok(current_desktop)
}

pub fn is_admin() -> Result<bool, String> {
    let status = std::process::Command::new("cmd")
        .args(["/C", "net", "session"])
        .spawn()
        .map_err(|_| "Cannot execute check command for administrator previligies".to_string())?
        .wait()
        .map_err(|_| "Cannot execute check command for administrator previligies".to_string())?;

    Ok(status.success())
}

pub fn restart_as_admin() -> Result<(), String> {
    std::process::Command::new("cmd").args(["/C", "runas"]);

    HWND::DESKTOP
        .ShellExecute(
            "runas",
            &process_path::get_executable_path()
                .ok_or("Cannot get process path".to_string())?
                .display()
                .to_string(),
            None,
            None,
            winsafe::co::SW::SHOWDEFAULT,
        )
        .map_err(|_| "Cannot start program with admin priveleges".to_string())?;
    Ok(())
}
