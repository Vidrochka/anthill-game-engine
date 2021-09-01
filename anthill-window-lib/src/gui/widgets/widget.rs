use crate::gui::draweable::TDraweable;
pub trait TWidget {
    fn draw_widget(&self);
}

impl TDraweable for dyn TWidget {
    fn draw(&self) {
        self.draw_widget();
    }
}

