use std::time::Duration;

pub struct Score(pub i32);

pub struct Lifespan {
    pub time_left: Duration,
    pub total_time: Duration,
}
