use sdl2::video::WindowBuilder;

use crate::window::window::Window;

pub trait TWindowBuildStrategy {
    fn init_draw_system(&self, video_subsystem: &mut sdl2::VideoSubsystem);
    fn build_window(&self, window_builder: WindowBuilder, video_subsystem: &mut sdl2::VideoSubsystem) -> Window;
}