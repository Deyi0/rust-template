// Enum //

#[derive(PartialEq)]
#[derive(Debug)]
pub enum ContextMode {
    Port, Dev
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Platform {
    Windows, Linux, Other
}

// Constants //

pub static APP_NAME: &str = "rust_template";
pub static CURRENT_CONTEXT: ContextMode = if cfg!(debug_assertions) { ContextMode::Dev } else { ContextMode::Port };

#[cfg(target_os = "windows")]
pub static CURRENT_PLATFORM: Platform = Platform::Windows;
#[cfg(target_os = "linux")]
pub static CURRENT_PLATFORM: Platform = Platform::Linux;
#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub static CURRENT_PLATFORM: Platform = Platform::Other;