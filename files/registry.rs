//! Windows registry read helpers for Mathworks Polyspace.

pub const APP_KEY: &str = "Software\\Mathworks\\Mathworks Polyspace";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Mathworks Polyspace";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Mathworks Polyspace") },
        RegValue { name: "Version",     value: String::from("v3.5.8") },
        RegValue { name: "Channel",     value: String::from("beta") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Mathworks Polyspace") },
        RegValue { name: "Publisher",       value: String::from("Mathworks") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Mathworks Polyspace\\uninstall.exe\"") },
    ]
}
