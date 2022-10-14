use std::time::Duration;

pub struct Score(pub i32);

pub struct Lifespan {
    pub time_left: Duration,
    pub total_time: Duration,
}

#[derive(Copy, Clone)]
pub struct SpeedBoost {
    pub is_boosting: bool,
    pub last_boost: Option<Duration>,
    pub time_left: Duration,
}
