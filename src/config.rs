// IMPORT //

use std::{env::consts::OS};

// ENUM //

#[derive(PartialEq)]
#[derive(Debug)]
pub enum ContextMode {
    Port, Dev
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Platform {
    Windows, Linux, MacOS, Android, IOS, Other
}

// CONSTANTS //

pub static APP_NAME: &str = "RustTemplate";
pub static CURRENT_CONTEXT: ContextMode = if cfg!(debug_assertions) { ContextMode::Dev } else { ContextMode::Port };

pub(crate) fn current_platform() -> Platform {
    match OS {
        "windows" => Platform::Windows,
        "macos" => Platform::MacOS,
        "linux" => Platform::Linux,
        "android" => Platform::Android,
        "ios" => Platform::IOS,
        _ => Platform::Other
    }
}