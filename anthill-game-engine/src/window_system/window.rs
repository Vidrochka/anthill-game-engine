use crate::system_dependent::window::window::WindowInfo;
use super::events::WindowSystemEvent;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub struct Window {
    window_info: WindowInfo,
    sdl_window: sdl2::video::Window,
    sdl_swap_render_context_callback: fn(&mut sdl2::video::Window),
    //event_map: HashMap<WindowSystemEvent, Vec<fn(event: WindowSystemEvent) -> bool>>
}

impl Window {
    pub fn new(window_info: WindowInfo, sdl_context: sdl2::Sdl, sdl_window: sdl2::video::Window, sdl_swap_render_context_callback: fn(&mut sdl2::video::Window)) -> Window {
        let event_map: HashMap<WindowSystemEvent, Vec<fn(event: WindowSystemEvent) -> bool>> = HashMap::new();

        //event_map.insert(WindowSystemEvent::KeyDown, Vec::new());
        
        Window 
        {
            window_info: window_info, 
            sdl_context: sdl_context, 
            sdl_window: sdl_window, 
            sdl_swap_render_context_callback: sdl_swap_render_context_callback,
            //event_map: event_map,
        }
    }

    pub fn wrap_to_arc_mutex(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

    pub fn show(&mut self) {
        self.sdl_window.show();
    }

    pub fn hide(&mut self) {
        self.sdl_window.hide();
    }

    pub fn swap_render_context(&mut self) {
        (self.sdl_swap_render_context_callback)(&mut self.sdl_window);
    }

    /*pub fn subscribe(&mut self, binding_callback: fn(bindings_map: &mut HashMap<WindowSystemEvent, Vec<fn(event: WindowSystemEvent) -> bool>>)) {
        binding_callback(& mut self.event_map);
    }*/

    pub fn process_events() {

    }
}