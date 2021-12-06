use crate::math::Vec2;

pub struct Transform {
    pub position: Vec2,
    pub rotation: Vec2,
}

impl Transform {
    pub const fn new(position: Vec2, rotation: Vec2) -> Self {
        Self { position, rotation }
    }
}
