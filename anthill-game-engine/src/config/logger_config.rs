use anthill_macros_lib::config_file;

#[config_file(file_path = "./configs/logger-config.json")]
pub struct LoggerConfig{
    pub file_path: String,
    pub with_console: bool,
    pub file_log_level: log::LevelFilter,
    pub console_log_level: log::LevelFilter,
    pub console_layout: String,
}