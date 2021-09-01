use std::sync::{Arc, Mutex};

use anthill_window_lib::gui::layout_builder::TLayoutBuilder;
use anthill_window_lib::gui::{
    draweable::TDraweable,
    mvc::TView,
    mvc::TController
};
use crate::gui::{
    controllers::new_project::NewProjectController,
    models::new_project::NewProject,
};

use crate::gui::views::new_project::NewProjectView;

#[derive(Default)]
pub struct NewProjectLayoutBuilder {
}

impl TLayoutBuilder for NewProjectLayoutBuilder {
    fn build_layout(&self) -> Box<dyn TDraweable> {
        let model = Arc::new(Mutex::new(NewProject::default()));

        let mut controller = NewProjectController::default();
        controller.set_model(Arc::clone(&model));
        
        let mut view = NewProjectView::default();
        view.initialize(Arc::clone(&model), controller);

        Box::new(Box::new(view) as Box<dyn TView<NewProjectController,NewProject>>)
    }
}