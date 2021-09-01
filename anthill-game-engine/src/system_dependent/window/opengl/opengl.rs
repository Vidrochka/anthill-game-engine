
use crate::system_dependent::window::window::{TWindowBuildStrategy,WindowInfo};
use sdl2::
{
    VideoSubsystem,
    video::
    {
        WindowBuilder, 
        Window, 
        GLProfile
    }
};

pub struct OpenGLWindowBuildStrategy {
}

impl OpenGLWindowBuildStrategy {
    pub fn new() -> Box<dyn TWindowBuildStrategy> {
        Box::new(OpenGLWindowBuildStrategy{})
    }
}

impl TWindowBuildStrategy for OpenGLWindowBuildStrategy {
    fn build_window(&mut self, video_subsystem: &mut VideoSubsystem) -> (WindowInfo, Window) {
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let mut window_builder = window_builder;
        let window = window_builder.opengl().build().unwrap();

        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

        (WindowInfo::OpengGL{ctx: gl_context}, window)
    }

    fn get_swap_render_context_callback(&mut self) -> fn(&mut sdl2::video::Window) {
        return |window:&mut  Window| -> _ {window.gl_swap_window()}
    }
}