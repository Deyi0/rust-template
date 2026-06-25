// IMPORT //

use global_hotkey::{HotKeyState, hotkey::{Code, HotKey, Modifiers}};
use serde::{Serialize, Deserialize};

// CLASS //

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputMethod { HotKey }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputTrigger { Press, Release, Dual }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputAction { Quit }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Input {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
    pub key: Code,
    
    pub method: InputMethod,
    pub trigger: InputTrigger,
    pub action: Option<InputAction>
}

impl InputTrigger {
    pub fn is_equal_state(&self, state: HotKeyState) -> bool {
        match self {
            InputTrigger::Press => state == HotKeyState::Pressed,
            InputTrigger::Release => state == HotKeyState::Released,
            InputTrigger::Dual => true
        }
    }
}

impl Input {
    pub fn new(shift: bool, ctrl: bool, alt: bool, meta: bool, key: Code, method: InputMethod, trigger: InputTrigger, action: Option<InputAction>) -> Self {
        Self { shift, ctrl, alt, meta, key, method, trigger, action }
    }
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        self.shift == other.shift
        && self.ctrl == other.ctrl
        && self.alt == other.alt
        && self.meta == other.meta
        && self.key == other.key
        && self.method == other.method
        && self.trigger == other.trigger
    }
}

impl From<HotKey> for Input {
    fn from(value: HotKey) -> Self {
        Input { 
            shift: value.mods.shift(),
            ctrl: value.mods.ctrl(),
            alt: value.mods.alt(),
            meta: value.mods.meta(),
            key: value.key,
            
            method: InputMethod::HotKey,
            trigger: InputTrigger::Press,
            action: None
        }
    }
}

impl Into<HotKey> for Input {
    fn into(self) -> HotKey {
        let mut mods = Modifiers::empty();
        
        if self.shift {
            mods |= Modifiers::SHIFT;
        } if self.ctrl {
            mods |= Modifiers::CONTROL;
        } if self.alt {
            mods |= Modifiers::ALT;
        } if self.meta {
            mods |= Modifiers::META;
        }
        
        if mods.is_empty() {
            HotKey::new(None, self.key)
        } else {
            HotKey::new(Some(mods), self.key)
        }
    }
}