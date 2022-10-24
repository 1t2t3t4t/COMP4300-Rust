use std::ops::{Add, Div, Mul, Sub};

use ggez::graphics::DrawParam;
use ggez::mint::Point2;

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
        let diff = rhs - self;
        diff.magnitude()
    }

    pub fn normalized(&self) -> Self {
        let length = self.magnitude();
        if length == 0f32 {
            Self::zero()
        } else {
            Vec2::new(self.x / length, self.y / length)
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl From<Point2<f32>> for Vec2 {
    fn from(p: Point2<f32>) -> Self {
        Self { x: p.x, y: p.y }
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

impl From<Vec2> for Point2<f32> {
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

impl<T> Add<T> for Vec2
where
    T: Into<f32>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let num = rhs.into();
        Self {
            x: self.x + num,
            y: self.y + num,
        }
    }
}

impl<T> Mul<T> for Vec2
where
    T: Into<f32>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let num = rhs.into();
        Self {
            x: self.x * num,
            y: self.y * num,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
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

impl Sub<f32> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
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

pub mod collision {
    use super::Vec2;

    pub struct BoxCollision {
        pos: Vec2,
        size: Vec2,
    }

    impl BoxCollision {
        pub fn new(pos: Vec2, size: Vec2) -> Self {
            Self { pos, size }
        }

        fn half_pos(&self) -> Vec2 {
            self.pos + self.size / 2f32
        }

        pub fn overlap_size(&self, b: impl Into<BoxCollision>) -> Option<Vec2> {
            let b_pos = b.into();

            let a_half_pos = self.half_pos();
            let b_half_pos = b_pos.half_pos();
            let a_size = self.size / 2f32;
            let b_size = b_pos.size / 2f32;

            let diff = (b_half_pos - a_half_pos).abs();
            let ox = a_size.x + b_size.x - diff.x;
            let oy = a_size.y + b_size.y - diff.y;

            if ox > 0f32 && oy > 0f32 {
                Some(Vec2::new(ox, oy))
            } else {
                None
            }
        }

        pub fn collide_aabb(&self, b: &BoxCollision) -> bool {
            self.horizontal_overlap(b) && self.vertical_overlap(b)
        }

        pub fn horizontal_overlap(&self, b: &BoxCollision) -> bool {
            self.pos.y <= (b.pos.y + b.size.y) && b.pos.y <= (self.pos.y + self.size.y)
        }

        pub fn vertical_overlap(&self, b: &BoxCollision) -> bool {
            self.pos.x <= (b.pos.x + b.size.x) && b.pos.x <= (self.pos.x + self.size.x)
        }
    }
}
