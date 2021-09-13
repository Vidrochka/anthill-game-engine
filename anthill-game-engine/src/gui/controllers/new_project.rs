use std::sync::{Arc, Mutex};

use anthill_di::Injection;
use anthill_window_lib::gui::mvc::TController;
use super::super::models::new_project::NewProject;

pub struct NewProjectController {
    model: Arc<Mutex<NewProject>>
}

impl Injection for NewProjectController {
    fn build_injection(injector: &mut anthill_di::Injector) -> Result<Self, anthill_di::DiError> {
        Ok(Self {
            model: injector.get_singletone()?,
        })
    }
}

impl TController<NewProject> for NewProjectController {
}
