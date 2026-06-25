// IMPORT //

use anyhow::Result;
use global_hotkey::hotkey::Code;
use serde::{Deserialize, Serialize};
use winit::{
    application::ApplicationHandler,
    event_loop::EventLoop
};

use crate::features::input_feature::classes::{Input, InputAction, InputMethod, InputTrigger};
use crate::features::input_feature::service::InputService;
use crate::modules::shared::{CustomEvent, CustomSettingsEvent};
use crate::features::settings_feature::{classes::Settings, service::SettingsService};

// CLASS //

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
struct AppSettings {
    pub inputs: Vec<Input>
}

struct App {
    settings_service: SettingsService<AppSettings>,
    input_service: InputService,
}

impl Settings for AppSettings {
    fn get_version(&self) -> u16 {
        0
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            inputs: vec![
                Input::new(
                    false, false, true, true,
                    Code::KeyC, InputMethod::HotKey, InputTrigger::Press, Some(InputAction::Quit)
                )
            ]
        }
    }
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            settings_service: SettingsService::new()?,
            input_service: InputService::new()?
        })
    }
}

impl ApplicationHandler<CustomEvent> for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.settings_service.init();
    }
    
    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: CustomEvent) {
        match event {
            CustomEvent::Input(event) => {
                if let Some(action) = self.input_service.run(&event) {
                    match action {
                        InputAction::Quit => {
                            event_loop.exit();
                        }
                    }
                }
            },
            
            CustomEvent::Settings(settings_event) => {
                match settings_event {
                    CustomSettingsEvent::Initialized | CustomSettingsEvent::Changed => { // Handle settings changes
                        for input in &self.settings_service.settings.inputs {
                            let _ = self.input_service.register_input(input);
                        }
                    }
                }
            }
        }
    }
    
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    )
    {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            
            _ => {}
        }
    }
    
    fn exiting(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let _ = self.settings_service.quit();
        let _ = self.input_service.quit();
    }
}

// PUBLIC //

pub fn run() -> Result<()> {
    let event_loop = EventLoop::<CustomEvent>::with_user_event().build()?;
    
    let proxy = event_loop.create_proxy();
    
    let mut app = App::new()?;
    
    app.settings_service.boot(proxy.clone());
    app.input_service.boot(proxy.clone());
    
    event_loop.run_app(&mut app)?;
    
    Ok(())
}