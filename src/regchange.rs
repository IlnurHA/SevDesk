use pyo3::prelude::*;

pub fn change_desktop_path(new_desktop_path: &String) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let py_script = PyModule::from_code(
            py,
            r#"
def change_desktop_path(new_desktop_path):
    import winreg

    path_to_desktop_value = "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\User Shell Folders"
    desktop_key = winreg.OpenKey(winreg.HKEY_CURRENT_USER, path_to_desktop_value, 0, winreg.KEY_WRITE)
    winreg.SetValueEx(desktop_key, "Desktop", 0, winreg.REG_EXPAND_SZ, new_desktop_path)
    winreg.CloseKey(desktop_key)
        "#,
            "winreg_py",
            "winreg_py",
        ).expect("Cannot read python script");
        py_script
            .getattr("change_desktop_path")
            .expect("Cannot get function 'change_desktop_name' from python script")
            .call1((new_desktop_path,))
            .expect("Cannot call py function");
    });
}
