// Mod //

mod app;
mod config;
mod core;
mod ui;
mod services;
mod utils;

// Import //

use std::{env, path::PathBuf, sync::OnceLock};

use crate::config::{CURRENT_CONTEXT, CURRENT_PLATFORM, ContextMode};

// Constant //

static RESOURCE_ROOT: OnceLock<PathBuf> = OnceLock::new();

// Helper //

fn resolve_resource_root() {
    let result = if CURRENT_CONTEXT == ContextMode::Dev {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("dev")
    } else {
        env::current_exe().expect("No exe path").parent().unwrap().to_path_buf()
    };
    
    let _ = RESOURCE_ROOT.set(result);
}

fn get_base_path() -> PathBuf {
    
    if let Some(path) = RESOURCE_ROOT.get() { return path.clone(); }
    
    match CURRENT_CONTEXT {
        ContextMode::Dev => PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        ContextMode::Port => env::current_exe()
            .ok()
            .and_then(|path| path.parent().map(|parent| parent.to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."))
    }
}

pub fn get_path(path: &str) -> PathBuf {
    get_base_path().join(path)
}

fn initial_log() {
    println!("Current context: {:?}", CURRENT_CONTEXT);
    println!("Platform: {:?}", CURRENT_PLATFORM);
    println!("Portable directory: {:?}", get_base_path());
}

// Main //

fn main() {
    resolve_resource_root();
    initial_log();
    
    app::run();
}