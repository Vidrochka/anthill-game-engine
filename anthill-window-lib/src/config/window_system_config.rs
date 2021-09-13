
#[derive(Debug, Clone, PartialEq)]
pub enum DrawApi {
    OpenGL
}

pub struct WindowSystemConfig {
    pub draw_api: DrawApi
}