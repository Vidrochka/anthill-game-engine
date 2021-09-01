use crate::gui::draweable::TDraweable;

pub trait TLayoutBuilder {
    fn build_layout(&self) -> Box<dyn TDraweable>;
}