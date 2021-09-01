use std::sync::{Arc, Mutex, MutexGuard};

use sdl2::{Sdl, VideoSubsystem};
use super::{WindowBuilder,Window};

pub struct WindowSystem {
    pub sdl_context: Arc<Sdl>,
    pub video_subsystem: Arc<VideoSubsystem>,
    pub window_collection: Vec<Arc<Mutex<Window>>>,
}

impl WindowSystem {
    pub fn new() -> Arc<Mutex<Self>> {
        log::info!("window system creating...");
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let ws = Arc::new(Mutex::new(Self {sdl_context: Arc::new(sdl_context), video_subsystem: Arc::new(video_subsystem), window_collection: Vec::new()}));
        log::info!("window system created");
        ws
    }

    pub fn swap_all_render_context(&mut self) {
        self.window_collection.iter().for_each(|window| window.lock().unwrap().swap_render_context() );
    }
}