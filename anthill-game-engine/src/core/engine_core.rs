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

pub struct EngineCore {
    time_tracker: Arc<Mutex<TimeTracker>>,
    //window_system: Arc<Mutex<WindowSystem>>,
    config: CoreConfig,
    log_handle: log4rs::Handle,
    is_closed: bool,
    //window: Arc<Mutex<Window>>,
}

impl EngineCore {
    pub async fn new() -> Result<EngineCore,String> {
        let log_handle = LoggerBuilder::build().await?;
        let config = tokio::spawn(async { CoreConfig::load().await });
        
        //let window_system = WindowSystem::new();

        //let window_builder = WindowBuilder::new(Arc::clone(&window_system));
        
        let config = config.await.map_err(|e|e.to_string())??;
        //let window = window_builder.build(config.render_api, WindowBuildOptions{lable: "Anthill game engine".to_string(), width: 800, height: 600});   
        //window.lock().map_err(|e|e.to_string())?.show();

        let time_tracker = TimeTracker::new().wrap_to_arc_mutex();

        let core = EngineCore
        {
            time_tracker: time_tracker,
            //window_system: window_system,
            config: config,
            log_handle: log_handle,
            is_closed: false,
            //window: window,
        };

        log::info!("Core created");

        Result::Ok(core)
    }
    
    pub fn run(&mut self) -> Result<(), String> {
        loop {
            self.time_tracker.lock().map_err(|e|e.to_string())?.new_step();
            let time_tracker = self.time_tracker.as_ref().lock().unwrap();

            log::info!("{}", time_tracker.get_fps());

            //self.window_system.lock().map_err(|e|e.to_string())?.swap_all_render_context();
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