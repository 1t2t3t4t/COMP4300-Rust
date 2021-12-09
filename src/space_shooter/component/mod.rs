use std::time::Duration;
use crate::common::Transform;
use crate::math::Vec2;
use crate::space_shooter::component::general::Score;
use crate::space_shooter::component::movement::Speed;
use crate::space_shooter::component::shape::{Geometry, Shape};
use crate::space_shooter::Tag;
use crate::{WINDOWS_HEIGHT, WINDOWS_WIDTH};
use ecs::entity::Entity;
use ecs::manager::EntityManager;
use rand;
use rand::Rng;
use crate::space_shooter::component::constant::{ENEMY_MAX_SPEED, ENEMY_MIN_SPEED, ENEMY_SIZE, ENEMY_SPAWN_INTERVAL};
use crate::space_shooter::component::game::Spawner;

pub(crate) mod constant {
    use std::time::Duration;

    pub const PLAYER_SPEED: f32 = 150f32;

    pub const ENEMY_MIN_SPEED: f32 = 100f32;
    pub const ENEMY_MAX_SPEED: f32 = 200f32;
    pub const ENEMY_SIZE: f32 = 32f32;

    pub const ENEMY_SPAWN_INTERVAL: Duration = Duration::from_secs(3);
}

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

pub fn create_enemy(manager: &mut EntityManager) -> &Entity {
    let mut rng = rand::thread_rng();
    let speed = rng.gen_range(ENEMY_MIN_SPEED..=ENEMY_MAX_SPEED);
    let x_pos = rng.gen_range(0f32..=(WINDOWS_WIDTH - ENEMY_SIZE));
    let y_pos = rng.gen_range(0f32..=(WINDOWS_HEIGHT - ENEMY_SIZE));

    manager
        .add_tag(Tag::Enemy)
        .add_component(Shape {
            geometry: Geometry::Circle,
            radius: ENEMY_SIZE,
        })
        .add_component(Transform::new(
            Vec2::new(x_pos, y_pos),
            Vec2::zero(),
        ))
        .add_component(Score(100))
        .add_component(Speed {
            velocity: Vec2::new(speed, speed),
        })
}

pub fn create_enemy_spawner(manager: &mut EntityManager) -> &Entity {
    manager.add_tag(Tag::Spawner)
        .add_component(Spawner {
            max: 20,
            interval: ENEMY_SPAWN_INTERVAL,
            last_spawned_duration: Duration::from_secs(0)
        })
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

pub mod general {
    pub struct Score(pub i32);
}

pub mod game {
    use std::time::Duration;

    pub struct Spawner {
        pub max: usize,
        pub interval: Duration,
        pub last_spawned_duration: Duration
    }
}
