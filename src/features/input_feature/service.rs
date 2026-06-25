// IMPORT //

use anyhow::Result;

use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, hotkey::HotKey};

use crate::modules::shared::*;
use super::classes::*;

// CLASS // 

pub struct InputService {
    manager: GlobalHotKeyManager,
    inputs: Vec<Input>
}

impl InputService {
    pub fn new() -> Result<Self> {
        let manager = GlobalHotKeyManager::new()?;
        let inputs = Vec::new();
        
        Ok(Self {
            manager,
            inputs
        })
    }
    
    pub fn boot(&self, proxy: Proxy) {
        std::thread::spawn(move || {
            while let Ok(event) = GlobalHotKeyEvent::receiver().recv() {
                proxy.send_event(CustomEvent::Input(CustomInputEvent::HotKeyTrigger(event))).unwrap();
            }
        });
    }
    
    pub fn run(&self, event: &CustomInputEvent) -> Option<InputAction> {
        match event {
            CustomInputEvent::HotKeyTrigger(event) => {
                for input in self.inputs.iter() {
                    
                    if !input.trigger.is_equal_state(event.state) { continue }
                    
                    let hotkey: HotKey = input.clone().into();
                    
                    if hotkey.id() != event.id() { continue }
                    
                    return input.action;
                }
            }
        }
        
        None
    }
    
    pub fn register_input(&mut self, input: &Input) -> Result<()> {
        match input.method {
            InputMethod::HotKey => {
                let hotkey: HotKey = input.clone().into();
                
                self.manager.register(hotkey)?;
            }
        }
        
        self.inputs.retain(|value| {
            *value != *input
        });
        
        self.inputs.push(input.clone());
        
        Ok(())
    }
    
    pub fn unregister_input(&mut self, input: &Input) -> Result<()> {
        match input.method {
            InputMethod::HotKey => {
                let hotkey: HotKey = input.clone().into();
                
                self.manager.unregister(hotkey)?;
            }
        }
        
        self.inputs.retain(|value| {
            *value != *input
        });
        
        Ok(())
    }
    
    pub fn quit(&mut self) -> Result<()> {
        for input in self.inputs.clone().iter() {
            self.unregister_input(input)?;
        }
        
        Ok(())
    }
}