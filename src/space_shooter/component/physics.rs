use crate::math::{collision::BoxCollision, Vec2};

#[derive(Clone, Copy)]
pub struct Collider {
    pub center: Vec2,
    pub radius: f32,
}

impl From<Collider> for BoxCollision {
    fn from(c: Collider) -> Self {
        BoxCollision::new(
            Vec2::new(c.center.x - c.radius, c.center.y - c.radius),
            Vec2::new(c.radius * 2f32, c.radius * 2f32),
        )
    }
}
