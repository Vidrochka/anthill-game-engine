use std::sync::{Arc, Mutex};

use sdl2::{VideoSubsystem, video::{GLContext, GLProfile, WindowBuilder}};

use super::super::{IWindowGapi, TWindowGapiBuilder, Window, WindowSystem, WindowBuildOptions};

pub struct OpenGLWindowGapi{
    gl_context: GLContext
}

impl OpenGLWindowGapi {
    pub fn new(ctx: GLContext) -> Self {
        Self {gl_context: ctx}
    }
}

impl IWindowGapi for OpenGLWindowGapi {
    fn swap_render_context(&self, window: &Window) {
        window.sdl_window.gl_swap_window();
    }
}

pub struct OpenGLWindowBuilder {
    window_system: Arc<Mutex<WindowSystem>>,
}

impl OpenGLWindowBuilder {
    pub fn new(window_system: Arc<Mutex<WindowSystem>>) -> Box<Self> {
        Box::new(Self {window_system: window_system})
    }
}

impl TWindowGapiBuilder for OpenGLWindowBuilder {
    fn build(&mut self, options: WindowBuildOptions) -> Arc<Mutex<Window>> {
        let video_subsystem = &self.window_system.lock().unwrap().video_subsystem;
        
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let mut window_builder = video_subsystem.window(options.lable.as_str(), options.width, options.height);
        window_builder.position_centered().allow_highdpi().hidden();
        let window = window_builder.opengl().build().unwrap();

        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

        Arc::new(Mutex::new(Window::new(window, Box::new(OpenGLWindowGapi::new(gl_context)))))
    }
} 