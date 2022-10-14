use std::time::Duration;

pub struct Spawner {
    pub max: usize,
    pub interval: Duration,
    pub last_spawned_duration: Duration,
}

pub struct Scoreboard {
    pub current_score: i32,
}

#[derive(Clone)]
pub struct DisplayTextEvent {
    pub text: String,
    pub dur: Duration,
}

#[derive(Default)]
pub struct DisplayText {
    pub texts: Vec<DisplayTextEvent>,
    pub cache: Option<CacheDisplayText>,
}

pub struct CacheDisplayText {
    pub raw_text: String,
    pub text: ggez::graphics::Text,
    pub position: [f32; 2],
}
