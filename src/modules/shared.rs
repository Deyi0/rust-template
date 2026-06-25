// IMPORT //

use global_hotkey::GlobalHotKeyEvent;
use winit::event_loop::EventLoopProxy;

// CLASS //

pub type Proxy = EventLoopProxy<CustomEvent>;

#[derive(Debug)]
pub enum CustomSettingsEvent { Initialized, Changed }
#[derive(Debug)]
pub enum CustomInputEvent { HotKeyTrigger(GlobalHotKeyEvent) }

#[derive(Debug)]
pub enum CustomEvent {
    Settings(CustomSettingsEvent),
    Input(CustomInputEvent)
}