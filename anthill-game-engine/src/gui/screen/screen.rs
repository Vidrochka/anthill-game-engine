use std::sync::{Arc, Mutex};

use anthill_di::{DiError, Injection};
use anthill_window_lib::gui::mvc::TView;
use anthill_window_lib::gui::{draweable::TDraweable, screen::TScreen};
use crate::core::state::{CoreState, State};
use crate::gui::controllers::new_project::NewProjectController;
use crate::gui::models::new_project::NewProject;

pub struct Screen {
    state: Arc<Mutex<CoreState>>,
    new_project_view: Box<dyn TView<NewProjectController,NewProject>>,
}

impl Injection for Screen {
    fn build_injection(injector: &mut anthill_di::Injector) -> Result<Self, DiError> {
        Ok(Self{
            state: injector.get_singletone()?,
            new_project_view: injector.get_new_instance()?,
        })
    }
}

impl TScreen for Screen {
    fn draw_frame(&self) {
        match self.state.lock().unwrap().get_state() {
            State::CreateProject => self.new_project_view.draw(),
        }
    }
}