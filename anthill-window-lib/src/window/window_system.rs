use std::sync::{Arc, Mutex};

use anthill_di::{DiError, Injection, builders::ContainerBuilder};

use crate::{
    config::window_system_config::{
        DrawApi, 
        WindowSystemConfig
    }, 
    window_build_strategy::{
        window_build_strategy::TWindowBuildStrategy,
        opengl_window_build_strategy::OpenglWindowBuildStrategy
    }
};

use super::{window::Window, window_build_options::WindowBuildOptions};


pub struct WindowSystem {
    #[allow(dead_code)]
    sdl_context: sdl2::Sdl,
    sdl_video_subsystem: sdl2::VideoSubsystem,
    window_build_strategy: Box<dyn TWindowBuildStrategy>,
    window_collection: Vec<Arc<Mutex<Window>>>,
    #[allow(dead_code)]
    config: WindowSystemConfig
}

impl Injection for WindowSystem {
    fn build_injection(injector: &mut anthill_di::Injector) -> Result<Self, anthill_di::DiError> {
        let sdl_context = sdl2::init().map_err(|e| DiError::CustomInjectTimeError(e))?;
        let mut sdl_video_subsystem = sdl_context.video().map_err(|e| DiError::CustomInjectTimeError(e))?;
        
        let config = injector.get_new_instance::<WindowSystemConfig>()?;

        match config.draw_api {
            DrawApi::OpenGL => {
                injector.add_container(ContainerBuilder::bind_interface::<dyn TWindowBuildStrategy, OpenglWindowBuildStrategy>().build());
            },
        }

        let window_build_strategy = injector.get_new_instance::<Box<dyn TWindowBuildStrategy>>()?;

        window_build_strategy.init_draw_system(&mut sdl_video_subsystem);

        Ok(WindowSystem {
            sdl_context: sdl_context,
            sdl_video_subsystem: sdl_video_subsystem,
            window_build_strategy: window_build_strategy,
            window_collection: Vec::new(),
            config: config,
        })
    }
}

impl WindowSystem {
    pub fn new_window(&mut self, options: WindowBuildOptions) -> Arc<Mutex<Window>> {
        let mut window_builder = self.sdl_video_subsystem.window(&*options.lable, options.width, options.height);
        window_builder.position_centered().allow_highdpi().hidden();
        let window
            = Arc::new(Mutex::new(self.window_build_strategy.build_window(window_builder, &mut self.sdl_video_subsystem)));
        self.window_collection.push(Arc::clone(&window));
        window
    }

    pub fn swap_all_render_context(&mut self) {
        self.window_collection.iter().for_each(|window| {window.lock().unwrap().swap_render_context()});
    }
}