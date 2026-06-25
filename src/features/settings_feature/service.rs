// IMPORT //

use std::io::Write;

use anyhow::Result;
use config::{Config, File};
use serde_json::to_string_pretty;

use crate::get_path;
use super::classes::Settings;
use crate::modules::shared::*;
use crate::modules::paths;

// CLASS //

pub struct SettingsService<T: Settings> {
    proxy: Option<Proxy>,
    
    pub settings: T
}

impl<T: Settings> SettingsService<T> {
    pub fn new() -> Result<Self> {
        let settings = Config::builder()
            .add_source(File::from(get_path(paths::SETTINGS_PATH)).required(false))
            .build()?
            .try_deserialize::<T>()?;
        
        Ok(Self {
            proxy: None,
            
            settings
        })
    }
    
    pub fn boot(&mut self, proxy: Proxy) {
        self.proxy = Some(proxy);
    }
    
    pub fn init(&self) {
        if let Some(proxy) = &self.proxy {
            proxy.send_event(CustomEvent::Settings(CustomSettingsEvent::Initialized)).unwrap();
        }
    }
    
    pub fn update(&mut self, callback: impl FnOnce(&mut T) -> ()) {
        callback(&mut self.settings);
        
        if let Some(proxy) = &self.proxy {
            proxy.send_event(CustomEvent::Settings(CustomSettingsEvent::Changed)).unwrap();
        }
    }
    
    pub fn save(&self) -> Result<()> {
        let json_string = to_string_pretty(&self.settings)?;
        let mut file = std::fs::File::create(get_path(paths::SETTINGS_PATH))?;
        
        file.write_all(json_string.as_bytes())?;
        
        Ok(())
    }
    
    pub fn quit(&self) -> Result<()> {
        self.save()
    }
}