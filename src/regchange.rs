use crate::py_scripts;
use pyo3::prelude::*;

pub fn change_desktop_path(new_desktop_path: &String) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let py_script = PyModule::from_code(
            py,
            py_scripts::CHANGE_DESKTOP_REGEDIT,
            "winreg_py",
            "winreg_py",
        )
        .expect("Cannot read python script");
        py_script
            .getattr("change_desktop_path")
            .expect("Cannot get function 'change_desktop_name' from python script")
            .call1((new_desktop_path,))
            .expect("Cannot call py function");
    });
}
pub fn get_current_desktop_path() -> Result<String, String> {
    pyo3::prepare_freethreaded_python();
    let py_result: PyResult<String> = Python::with_gil(|py| {
        let py_script = PyModule::from_code(
            py,
            py_scripts::CURRENT_DESKTOP_PATH,
            "winreg_py",
            "winreg_py",
        )?;
        py_script.getattr("change_desktop_path")?.call0()?;
        py_script.extract()
    });

    py_result.map_err(|_| String::from("Cannot read current desktop path"))
}
