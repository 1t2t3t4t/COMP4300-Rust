use std::ops::{Add, Mul, Sub};

use ggez::graphics::DrawParam;

#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub fn magnitude_sq(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.magnitude_sq())
    }

    pub fn distance(self, rhs: Self) -> f32 {
        let Self { x: dx, y: dy } = rhs - self;
        Vec2::new(dx, dy).magnitude()
    }

    pub fn normalized(&self) -> Self {
        let length = self.magnitude();
        if length == 0f32 {
            Self::zero()
        } else {
            Vec2::new(self.x / length, self.y / length)
        }
    }
}

impl From<Vec2> for DrawParam {
    fn from(vec2: Vec2) -> Self {
        ([vec2.x, vec2.y],).into()
    }
}

impl From<Vec2> for [f32; 2] {
    fn from(vec2: Vec2) -> Self {
        [vec2.x, vec2.y]
    }
}

impl From<Vec2> for ggez::mint::Point2<f32> {
    fn from(vec: Vec2) -> Self {
        let arr: [f32; 2] = vec.into();
        arr.into()
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

pub mod random {
    use rand::Rng;

    pub fn rand_element<T: Copy + Clone, const N: usize>(elements: [T; N]) -> T {
        let idx = rand::thread_rng().gen_range(0..elements.len());
        elements[idx]
    }
}
