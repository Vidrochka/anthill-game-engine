use crate::config::LoggerConfig;
use log::{debug, LevelFilter};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    encode::json::JsonEncoder,
    filter::threshold::ThresholdFilter,
    Handle,
};

pub struct LoggerBuilder {
}

impl LoggerBuilder {
    pub fn build(config: LoggerConfig) -> Result<Handle,String> {
        //let config = LoggerConfig::load()?;

        let mut log_config_builder = Config::builder()
            .appender
            (
                Appender::builder()
                .filter(
                    Box::new(ThresholdFilter::new(config.file_log_level))
                )
                .build(
                    "log_file",
                    Box::new(
                        FileAppender::builder().encoder(Box::new(JsonEncoder::new())).build(&config.file_path).map_err(|e|e.to_string())?
                    )
                )
            );
        let mut log_root_builder = Root::builder().appender("log_file");

        if config.with_console {
            log_config_builder = log_config_builder.appender
            (
                Appender::builder()
                .filter(
                    Box::new(ThresholdFilter::new(config.console_log_level))
                )
                .build(
                    "stdout", 
                    Box::new(ConsoleAppender::builder().encoder(Box::new(PatternEncoder::new(&*config.console_layout))).target(Target ::Stdout).build())
                )
            );
            log_root_builder = log_root_builder.appender("stdout");
        }

        let log_config = log_config_builder.build(log_root_builder.build(LevelFilter::Trace)).map_err(|e|e.to_string())?;

        let log_handle = log4rs::init_config(log_config).map_err(|e|e.to_string())?;

        debug!("log created");

        Result::Ok(log_handle)
    }
}