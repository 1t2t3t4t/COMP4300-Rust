use std::time::Duration;

pub struct Spawner {
    pub max: usize,
    pub interval: Duration,
    pub last_spawned_duration: Duration,
}
