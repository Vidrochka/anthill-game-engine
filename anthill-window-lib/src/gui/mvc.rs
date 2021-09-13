use crate::gui::draweable::TDraweable;

pub trait TController<M>  {
}

pub trait TView<C,M> where C: TController<M> {
    fn build_view(&mut self);

    fn draw_view(&self);
}

impl<C: TController<M>,M> TDraweable for Box<dyn TView<C,M>> {
    fn draw(&self) {
        self.draw_view();
    }
}