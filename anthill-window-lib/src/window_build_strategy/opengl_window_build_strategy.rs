use anthill_di::Injection;
use sdl2::video::{GLProfile, WindowBuilder};

use crate::window::window::Window;

use super::window_build_strategy::TWindowBuildStrategy;

pub struct OpenglWindowBuildStrategy {
}

impl Injection for OpenglWindowBuildStrategy {
    fn build_injection(_: &mut anthill_di::Injector) -> Result<Self, anthill_di::DiError> {
        Ok(Self{})
    }
}

impl TWindowBuildStrategy for OpenglWindowBuildStrategy {
    fn init_draw_system(&self, video_subsystem: &mut sdl2::VideoSubsystem) {
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);
    }

    fn build_window(&self, window_builder: WindowBuilder, video_subsystem: &mut sdl2::VideoSubsystem) -> Window {
        let mut window_builder = window_builder;
        let sdl_window = window_builder.opengl().build().unwrap();
        let gl_context = sdl_window.gl_create_context().unwrap();
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
        Window::new(gl_context, sdl_window, |sdl_window: &mut sdl2::video::Window| sdl_window.gl_swap_window())
    }
}
