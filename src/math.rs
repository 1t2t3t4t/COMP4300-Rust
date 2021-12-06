use std::ops::{Add, Mul, Sub};

use ggez::graphics::DrawParam;

#[derive(Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn zero() -> Self {
        Self { x: 0f32, y: 0f32 }
    }

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.x.powi(2) + self.y.powi(2))
    }

    pub fn distance(&self, rhs: &Self) -> f32 {
        let dx = self.x - rhs.x;
        let dy = self.y - rhs.y;
        Vec2::new(dx, dy).magnitude()
    }

    pub fn normalized(&self) -> Self {
        let length = self.magnitude();
        Vec2::new(self.x / length, self.y / length)
    }
}

impl From<Vec2> for DrawParam {
    fn from(vec2: Vec2) -> Self {
        ([vec2.x, vec2.y],).into()
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
