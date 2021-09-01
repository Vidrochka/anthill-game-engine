use anthill_macros_lib::config_file;
pub use crate::gui::types::render_api::RenderApi;

#[config_file(file_path = "./configs/core-config.json")]
pub struct CoreConfig{
    pub render_api: RenderApi,
}