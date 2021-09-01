use std::sync::{Arc, Mutex};
use chrono::prelude::*;

pub struct TimeTracker {
    track_time: DateTime<Utc>,
    last_step_duration: chrono::Duration,
    fps: f32
}

impl TimeTracker {
    pub fn new() -> TimeTracker {
        log::info!("time tracker initializing...");
        let tt = TimeTracker {
            track_time: Utc::now(),
            last_step_duration: chrono::Duration::seconds(1) / 60,
            fps: 60.0};
        log::info!("time tracker initialized");
        tt
    }

    pub fn new_step(&mut self) {
        let now = Utc::now();

        self.last_step_duration = now - self.track_time;
        self.track_time = now;

        let new_step_weight = 3.0;

        let new_step_fps = (1.0 / (self.last_step_duration.num_milliseconds() as f32 / 1000.0) ) as f32;
        self.fps = (self.fps + (new_step_fps * new_step_weight)) / (new_step_weight + 1.0);
    }

    pub fn get_last_step_duration(&self) -> chrono::Duration { self.last_step_duration }
    pub fn get_fps(&self) -> f32 { self.fps }

    pub fn wrap_to_arc_mutex(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }
}
