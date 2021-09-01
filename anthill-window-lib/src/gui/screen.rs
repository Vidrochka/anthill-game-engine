use std::sync::{Arc, Mutex};

use crate::gui::draweable::TDraweable;

pub trait TScreen {
    /// Строим интерфейсы
    ///
    /// # Example
    /// ```
    /// struct TestLayout {}
    /// 
    /// impl TLayoutBuilder for TestLayout {
    ///     pub fn build_layout(&self) -> Box<dyn TDraweable> {*something*}
    /// }
    ///
    /// struct TestDrawSystem {
    ///     main_layout: Option<Box<dyn TDraweable>>;  
    /// }
    ///
    /// impl TScreen for TestDrawSystem {
    ///     pub fn init(&mut self) {
    ///         let main_layout_builder = TestLayout{};
    ///         self.main_layout = main_layout_builder.build_layout();
    ///     }
    /// }
    /// 
    /// ```
    fn init(&mut self);

    /// Выбираем интерфейс
    ///
    /// # Example
    /// ```
    /// struct TestLayout {}
    /// 
    /// impl TLayoutBuilder for TestLayout {
    ///     pub fn build_layout(&self) -> Box<dyn TDraweable> {*something*}
    /// }
    ///
    /// struct TestDrawSystem {
    ///     main_layout: Option<Box<dyn TDraweable>>;  
    /// }
    ///
    /// impl TScreen for TestDrawSystem {
    ///     pub fn init(&mut self) {
    ///         let main_layout_builder = TestLayout{};
    ///         self.main_layout = main_layout_builder.build_layout();
    ///     }
    ///     pub fn get_screen(&self) -> Option<Box<dyn TDraweable>> {
    ///         if(*something*) { self.main_layout } else { None }
    ///     }
    /// }
    ///
    /// ```
    fn get_screen(&self) -> &Option<Box<dyn TDraweable>>;

    /// Рисуем кадр
    fn draw_frame(&self) {
        if let Some(layout) = self.get_screen() {
            layout.draw()
        }
    }
}