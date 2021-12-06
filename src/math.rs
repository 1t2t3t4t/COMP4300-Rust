use std::ops::Add;

use ggez::graphics::DrawParam;

#[derive(Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub const fn zero() -> Self {
        Self {
            x: 0f32, y: 0f32
        }
    }

    pub const fn new(x: f32, y: f32) -> Self {
        Self {
            x, y
        }
    }
}

impl From<Vec2> for DrawParam {
    fn from(vec2: Vec2) -> Self {
        ([vec2.x, vec2.y], ).into()
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}