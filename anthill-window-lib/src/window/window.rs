use sdl2::video::GLContext;

use crate::gui::screen::TScreen;


pub struct Window {
    #[allow(dead_code)]
    gl_context: GLContext,
    sdl_window: sdl2::video::Window,
    swap_render_context_callback: fn(&mut sdl2::video::Window),
    screen: Option<Box<dyn TScreen>>
}



impl Window {
    pub fn new(gl_context: GLContext, sdl_window: sdl2::video::Window, swap_render_context_callback: fn(&mut sdl2::video::Window)) -> Self {
        Self{
            gl_context: gl_context, 
            sdl_window: sdl_window, 
            swap_render_context_callback: swap_render_context_callback, 
            screen: None
        }
    }

    pub fn show(&mut self) {
        self.sdl_window.show();
    }
    
    pub fn hide(&mut self) {
        self.sdl_window.hide();
    }

    pub fn swap_render_context(&mut self) {
        if let Some(screen) = &self.screen {
            screen.draw_frame();
        }
        
        (self.swap_render_context_callback)(&mut self.sdl_window);
    }

    pub fn set_screen(&mut self, screen: Box<dyn TScreen>) {
        self.screen = Some(screen);
    }
}