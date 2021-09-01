use std::sync::{Arc, Mutex};
use crate::gui::types::RenderApi;
use super::Window;
use super::gapi::opengl::OpenGLWindowBuilder;
use super::{WindowSystem, TWindowGapiBuilder, WindowBuildOptions};


pub struct WindowBuilder {
    window_system: Arc<Mutex<WindowSystem>>,
}

impl WindowBuilder {
    pub fn new(window_system: Arc<Mutex<WindowSystem>>) -> Self {
        log::info!("window builder creating...");
        let wb = Self {window_system: window_system};
        log::info!("window builder created...");
        wb
    }
    
    pub fn build(self, render_api: RenderApi, options: WindowBuildOptions) -> Arc<Mutex<Window>> {
        log::info!("window creating...");

        let mut window_builder: Box<dyn TWindowGapiBuilder> = match render_api {
            RenderApi::OpenGL => {
                OpenGLWindowBuilder::new(Arc::clone(&self.window_system))
            },
        };

        let window = window_builder.build(options);
        self.window_system.lock().unwrap().window_collection.push(Arc::clone(&window));

        log::info!("window created");

        window
    }
}