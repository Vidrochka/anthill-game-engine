use std::sync::{Arc, Mutex};

use anthill_di::Injection;
use anthill_window_lib::gui::mvc::TView;
use super::super::models::new_project::NewProject;
use super::super::controllers::new_project::NewProjectController;

pub struct NewProjectView {
    model: Arc<Mutex<NewProject>>,
    controller: NewProjectController,
}

impl Injection for NewProjectView {
    fn build_injection(injector: &mut anthill_di::Injector) -> Result<Self, anthill_di::DiError> {
        let mut view = Self { 
            model: injector.get_singletone()?,
            controller: injector.get_new_instance()?,
        };
        
        view.build_view();

        Ok(view)
    }
}

impl TView<NewProjectController,NewProject> for NewProjectView {
    fn build_view(&mut self) {
    }

    fn draw_view(&self) {
        log::info!("draw")
    }
}