use std::sync::{Arc, Mutex};

use anthill_window_lib::gui::{draweable::TDraweable, layout_builder::TLayoutBuilder, screen::TScreen};
use crate::gui::layouts::new_project::NewProjectLayoutBuilder;

enum State {
    CreateProject
}

struct Screen {
    state: State,
    new_project_layout: Option<Box<dyn TDraweable>>,
}

impl Screen {
    fn new(state: State) -> Self {
        Self {state: state, new_project_layout: None}
    }
}

impl TScreen for Screen {
    fn init(&mut self) {
        let new_project_layout_builder = NewProjectLayoutBuilder::default();
        let  new_project_layout = new_project_layout_builder.build_layout();
        self.new_project_layout = Some(new_project_layout);
    }

    fn get_screen(&self) -> &Option<Box<dyn TDraweable>> {
        match self.state {
            State::CreateProject => &self.new_project_layout,
        }
    }
}