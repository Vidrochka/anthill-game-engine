use anthill_game_engine::core::engine_core::EngineCore;
//use log::{debug, error, info, trace, warn};

#[tokio::main]
pub async fn main() -> Result<(), String> {
    let mut core = EngineCore::new().await?;
    core.run()
}