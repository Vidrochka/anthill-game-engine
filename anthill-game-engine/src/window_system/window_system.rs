use crate::system_dependent::window::window::TWindowBuildStrategy;
use std::sync::{Arc, Mutex};
use super::Window;

pub struct WindowSystem {
    window_collection: Vec<Arc<Mutex<Window>>>,
}

impl WindowSystem {
    pub fn new() -> WindowSystem {
        WindowSystem {window_collection: Vec::new()}
    }

    pub fn new_window(&mut self, lable: &str, width: u32, height: u32, window_build_strategy: Box<dyn TWindowBuildStrategy>) -> Arc<Mutex<Window>> {
        let sdl_context = sdl2::init().unwrap();
        let mut video_subsystem = sdl_context.video().unwrap();

        let mut window_builder = video_subsystem.window(lable, width, height);
        window_builder.position_centered().allow_highdpi().hidden();

        let mut window_build_strategy = window_build_strategy;
        let build_res = window_build_strategy.build_window(window_builder, &mut video_subsystem);
        let window_ref = Window::new
        (
            build_res.0, 
            sdl_context,
            build_res.1, 
            window_build_strategy.get_swap_render_context_callback()
        ).wrap_to_arc_mutex();
        let winodw_ref_copy = window_ref.clone();

        self.window_collection.push(window_ref);
        winodw_ref_copy
    }

    pub fn swap_all_render_context(&mut self) {
        self.window_collection.iter().for_each(|window| {window.lock().unwrap().swap_render_context()});
    }
}