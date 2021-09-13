use derive_builder::Builder;

#[derive(Default, Debug, Builder)]
pub struct WindowBuildOptions {
    pub lable: String,
    pub width: u32, 
    pub height: u32,
}