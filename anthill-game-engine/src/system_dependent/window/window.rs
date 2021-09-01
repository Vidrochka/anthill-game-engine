
use sdl2::
{
    VideoSubsystem,
    video::
    {
        WindowBuilder, 
        Window,
        GLContext,
    }
};

pub trait TWindowBuildStrategy {
    fn build_window(&mut self, window_builder: WindowBuilder, video_subsystem: &mut VideoSubsystem) -> (WindowInfo, Window);
    fn get_swap_render_context_callback(&mut self) -> fn(&mut sdl2::video::Window);
}

pub enum WindowInfo {
    OpengGL{ctx: GLContext}
}