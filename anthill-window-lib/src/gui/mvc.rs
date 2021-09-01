use std::{
    sync::{Arc, Mutex},
};

use crate::gui::draweable::TDraweable;

/*pub trait TModel {
}*/

pub trait TController<M> /*where M: TModel*/ {
    fn set_model(&mut self, model: Arc<Mutex<M>>);
}

pub trait TView<C,M> where /*M: TModel,*/ C: TController<M> {
    fn initialize(&mut self, model: Arc<Mutex<M>>, controller: C) {
        self.set_model(model);
        self.set_controller(controller);
        self.build_view();
    }

    fn set_model(&mut self, model: Arc<Mutex<M>>);
    fn set_controller(&mut self, controller: C);
    fn build_view(&mut self);

    fn draw_view(&self);   
}

impl<C: TController<M>,M> TDraweable for Box<dyn TView<C,M>> /*where /*M: TModel,*/ C:*/  {
    fn draw(&self) {
        self.draw_view();
    }
}
