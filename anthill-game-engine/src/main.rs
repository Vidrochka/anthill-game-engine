use anthill_game_engine::{
    core::engine_core::EngineCore,
    utils::containers_builder::ContainersBuilder
};
use anthill_di::Injector;
//use log::{debug, error, info, trace, warn};

pub fn main() -> Result<(), String> {
    let injector = Injector::new(ContainersBuilder::construct());
    let mut core = injector.lock().unwrap().get_new_instance::<EngineCore>().map_err(|e| {
        log::error!("{:?}",e);
        format!("{:?}",e)
    })?;
    core.run()
}