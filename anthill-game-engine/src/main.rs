use anthill_game_engine::{config::{CoreConfig, LoggerConfig}, core::engine_core::EngineCore, utils::{logger_builder::LoggerBuilder, time::TimeTracker}};
use anthill_di::{
    Injector,
    builders::ContainerBuilder,
    DiError
};
//use log::{debug, error, info, trace, warn};

pub fn main() -> Result<(), String> {
    let containers = vec![
        ContainerBuilder::bind_unconfigured_type::<LoggerConfig>()
            .build_with_constructor(|_| -> _ {LoggerConfig::load().map_err(|e| DiError::CustomInjectTimeError(e))}),
        ContainerBuilder::bind_unconfigured_type::<CoreConfig>()
            .build_with_constructor(|_| -> _ {CoreConfig::load().map_err(|e| DiError::CustomInjectTimeError(e))}),
        ContainerBuilder::bind_unconfigured_type::<log4rs::Handle>()
            .build_with_constructor(|injector| -> _ { LoggerBuilder::build(injector.get_new_instance()?).map_err(|e| DiError::CustomInjectTimeError(e)) }),
        ContainerBuilder::bind_type::<TimeTracker>().build(),
        ContainerBuilder::bind_type::<EngineCore>().build()
    ];
    let injector = Injector::new(containers);
    let mut core = injector.lock().unwrap().get_new_instance::<EngineCore>().map_err(|e|format!("{:?}",e))?;
    core.run()
}