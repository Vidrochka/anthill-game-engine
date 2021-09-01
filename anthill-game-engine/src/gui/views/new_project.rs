use std::sync::{Arc, Mutex};

use anthill_window_lib::gui::mvc::TView;
use super::super::models::new_project::NewProject;
use super::super::controllers::new_project::NewProjectController;

#[derive(Default)]
pub struct NewProjectView {
    model: Option<Arc<Mutex<NewProject>>>,
    controller: Option<NewProjectController>,
}

impl TView<NewProjectController,NewProject> for NewProjectView {
    fn set_model(&mut self, model: Arc<Mutex<NewProject>>) {
        self.model = Some(model);
    }

    fn set_controller(&mut self, controller: NewProjectController) {
        self.controller = Some(controller);
    }

    fn build_view(&mut self) {
    }

    fn draw_view(&self) {
    }
}