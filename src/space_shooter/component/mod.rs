use crate::common::Transform;
use crate::math::Vec2;
use crate::space_shooter::component::shape::{Geometry, Shape};
use crate::space_shooter::Tag;
use crate::{WINDOWS_HEIGHT, WINDOWS_WIDTH};
use ecs::entity::Entity;
use ecs::manager::EntityManager;

pub fn create_player(manager: &mut EntityManager) -> &Entity {
    manager
        .add_tag(Tag::Player)
        .add_component(Shape {
            geometry: Geometry::Rectangle,
            radius: 32f32,
        })
        .add_component(Transform::new(
            Vec2::new(WINDOWS_WIDTH / 2f32 - 32f32, WINDOWS_HEIGHT / 2f32 - 32f32),
            Vec2::zero(),
        ))
}

pub mod shape {
    pub enum Geometry {
        Triangle,
        Rectangle,
        Circle,
    }

    pub struct Shape {
        pub geometry: Geometry,
        pub radius: f32,
    }
}

pub mod movement {
    use crate::math::Vec2;

    pub struct Speed {
        pub velocity: Vec2,
    }
}

pub mod physics {
    use crate::math::Vec2;

    pub struct Collider {
        pub center: Vec2,
        pub radius: f32,
    }
}
