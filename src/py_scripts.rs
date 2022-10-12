pub const CHANGE_DESKTOP_REGEDIT: &str = r#"
def change_desktop_path(new_desktop_path):
    import winreg

    path_to_desktop_value = "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\User Shell Folders"
    desktop_key = winreg.OpenKey(winreg.HKEY_CURRENT_USER, path_to_desktop_value, 0, winreg.KEY_WRITE)
    winreg.SetValueEx(desktop_key, "Desktop", 0, winreg.REG_EXPAND_SZ, new_desktop_path)
    winreg.CloseKey(desktop_key)
        "#;

pub const ELEVATION: &str = r#"
def elevation():
    import ctypes, sys

    def is_admin():
        try:
            return ctypes.windll.shell32.IsUserAnAdmin()
        except:
            return False

    if not is_admin():
        # Re-run the program with admin rights
        ctypes.windll.shell32.ShellExecuteW(None, "runas", sys.executable, " ".join(sys.argv), None, 1)
        exit(0)
        "#;

pub const CURRENT_DESKTOP_PATH: &str = r#"
def get_current_desktop_path():
    import winreg

    path_to_desktop_value = "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\User Shell Folders"
    desktop_key = winreg.OpenKey(winreg.HKEY_CURRENT_USER, path_to_desktop_value, 0, winreg.KEY_READ)

    i = 0

    while True:
        (key, value, _) = winreg.EnumValue(desktop_key, i)
        if key == 'Desktop':
            winreg.CloseKey(desktop_key)
            return value
        i += 1
"#;

pub const REBOOT_EXPLORER: &str = r#"
def reboot_explorer():
    import os

    os.system("taskkill /f /im explorer.exe")
    os.system("start explorer.exe")
    "#;
