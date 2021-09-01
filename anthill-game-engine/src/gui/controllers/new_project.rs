use std::sync::{Arc, Mutex};

use anthill_window_lib::gui::mvc::TController;
use super::super::models::new_project::NewProject;

#[derive(Default)]
pub struct NewProjectController {
    model: Option<Arc<Mutex<NewProject>>>
}

impl TController<NewProject> for NewProjectController {
    fn set_model(&mut self, model: Arc<Mutex<NewProject>>) {
        self.model = Some(model);
    }
}
