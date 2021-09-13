use crate::utils::{
    logger_builder::LoggerBuilder,
    time::TimeTracker
};
use crate::gui::{
    //types::render_api::RenderApi,
    //window::{WindowSystem, WindowBuildOptions, WindowBuilder, Window}
};
use crate::config::CoreConfig;
use std::sync::{Arc,Mutex};

use anthill_di::{DiError, Injection};
use anthill_window_lib::window::window::Window;
use anthill_window_lib::window::window_build_options::WindowBuildOptionsBuilder;
use anthill_window_lib::window::window_system::WindowSystem;

pub struct EngineCore {
    time_tracker: Arc<Mutex<TimeTracker>>,
    window_system: Arc<Mutex<WindowSystem>>,
    window: Arc<Mutex<Window>>,
    config: CoreConfig,
    log_handle: log4rs::Handle,
    is_closed: bool,
}

impl Injection for EngineCore {
    fn build_injection(injector: &mut anthill_di::Injector) -> Result<Self, anthill_di::DiError> {
        let log_handler = injector.get_new_instance()?;
        let window_system = injector.get_singletone::<WindowSystem>()?;
        let window = window_system.lock()
            .map_err(|e| DiError::CustomInjectTimeError(e.to_string()))?
            .new_window(
                WindowBuildOptionsBuilder::default()
                .lable("Anthill game engine".to_string())
                .height(600)
                .width(800)
                .build().map_err(|e|DiError::CustomInjectTimeError(e.to_string()))?
            );

        window.lock().map_err(|e|DiError::CustomInjectTimeError(e.to_string()))?
            .set_screen(injector.get_new_instance()?);
        window.lock().map_err(|e|DiError::CustomInjectTimeError(e.to_string()))?
            .show();

        let core = Self {
            log_handle: log_handler,
            window_system: window_system,
            window: window,
            time_tracker: injector.get_singletone()?,
            config: injector.get_new_instance()?,    
            is_closed: false
        };

        log::info!("Core created");
        
        Ok(core)
    }
}

impl EngineCore {
    pub fn run(&mut self) -> Result<(), String> {
        loop {
            self.time_tracker.lock().map_err(|e|e.to_string())?.new_step();
            let time_tracker = self.time_tracker.as_ref().lock().unwrap();

            //log::info!("{}", time_tracker.get_fps());

            self.window_system.lock().map_err(|e|e.to_string())?.swap_all_render_context();
            if self.is_closed() {
                break;
            }
        }

        log::info!("Engine shuting down");
        Result::Ok(())
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }
}