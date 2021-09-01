use std::sync::{Arc, Mutex};
use sdl2::video::WindowBuilder;


pub struct Window {
    pub sdl_window: sdl2::video::Window,
    pub window_gapi: Box<dyn IWindowGapi>
}

impl Window {
    pub fn new(sdl_window: sdl2::video::Window, Window_gapi: Box<dyn IWindowGapi>) -> Window {
        Window{sdl_window: sdl_window, window_gapi: Window_gapi}
    }

    pub fn show(&mut self) {
        self.sdl_window.show();
        log::info!("window showed");
    }

    pub fn hide(&mut self) {
        self.sdl_window.hide();
    }

    pub fn swap_render_context(&self) {
        &self.window_gapi.swap_render_context(self);
    }
}

pub trait IWindowGapi {
    fn swap_render_context(&self, window: &Window);
}

pub trait TWindowGapiBuilder {
    fn build(&mut self, options: WindowBuildOptions) -> Arc<Mutex<Window>>;
}

pub struct WindowBuildOptions{
    pub lable: String,
    pub width: u32, 
    pub height: u32,
}