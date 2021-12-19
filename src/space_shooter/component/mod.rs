use crate::common::GameTransform;
use crate::math::Vec2;
use crate::space_shooter::component::general::{Lifespan, Score};
use crate::space_shooter::component::movement::Speed;
use crate::space_shooter::component::shape::{Geometry, Shape};
use crate::space_shooter::Tag;
use crate::{WINDOWS_HEIGHT, WINDOWS_WIDTH};
use ecs::entity::Entity;
use ecs::manager::EntityManager;
use std::time::Duration;

use crate::math::random::rand_element;
use crate::space_shooter::component::constant::{
    BULLET_LIFESPAN, BULLET_SIZE, BULLET_SPAWN_INTERVAL, ENEMY_MAX_SPEED, ENEMY_MIN_SPEED,
    ENEMY_SIZE, ENEMY_SPAWN_INTERVAL, MAX_ENEMY_SPAWN,
};
use crate::space_shooter::component::game::Spawner;
use crate::space_shooter::component::physics::Collider;
use rand::Rng;

use self::game::Scoreboard;

pub mod game;
pub mod general;
pub mod movement;
pub mod physics;
pub mod shape;

pub(crate) mod constant {
    use std::time::Duration;

    pub const PLAYER_SPEED: f32 = 300f32;

    pub const BULLET_SIZE: f32 = 12f32;
    pub const BULLET_SPEED: f32 = 400f32;
    pub const BULLET_LIFESPAN: Duration = Duration::from_secs(2);
    pub const BULLET_SPAWN_INTERVAL: Duration = Duration::from_millis(300);

    pub const ENEMY_MIN_SPEED: f32 = 100f32;
    pub const ENEMY_MAX_SPEED: f32 = 200f32;
    pub const ENEMY_SIZE: f32 = 32f32;

    pub const MAX_ENEMY_SPAWN: usize = 32;
    pub const ENEMY_SPAWN_INTERVAL: Duration = Duration::from_secs(3);
}

pub fn create_bullet(
    manager: &mut EntityManager,
    speed: Speed,
    transform: GameTransform,
) -> &Entity {
    manager
        .add_tag(Tag::Bullet)
        .add_component(Shape {
            geometry: Geometry::Circle,
            radius: BULLET_SIZE,
        })
        .add_component(speed)
        .add_component(transform)
        .add_component(Lifespan {
            time_left: BULLET_LIFESPAN,
            total_time: BULLET_LIFESPAN,
        })
}

pub fn create_player(manager: &mut EntityManager) -> &Entity {
    manager
        .add_tag(Tag::Player)
        .add_component(Shape {
            geometry: Geometry::Rectangle,
            radius: 32f32,
        })
        .add_component(GameTransform::new(
            Vec2::new(WINDOWS_WIDTH / 2f32 - 32f32, WINDOWS_HEIGHT / 2f32 - 32f32),
            Vec2::zero(),
        ))
        .add_component(Collider {
            center: Vec2::new(WINDOWS_WIDTH / 2f32 - 32f32, WINDOWS_HEIGHT / 2f32 - 32f32),
            radius: 32f32,
        })
}

pub fn create_enemy(manager: &mut EntityManager) -> &Entity {
    let mut rng = rand::thread_rng();
    let speed = rng.gen_range(ENEMY_MIN_SPEED..=ENEMY_MAX_SPEED);
    let x_pos = rng.gen_range(0f32..=(WINDOWS_WIDTH - ENEMY_SIZE));
    let y_pos = rng.gen_range(0f32..=(WINDOWS_HEIGHT - ENEMY_SIZE));
    let shape = rand_element([Geometry::Rectangle, Geometry::Circle]);
    manager
        .add_tag(Tag::Enemy)
        .add_component(Shape {
            geometry: shape,
            radius: ENEMY_SIZE,
        })
        .add_component(GameTransform::new(Vec2::new(x_pos, y_pos), Vec2::zero()))
        .add_component(Score(100))
        .add_component(Speed {
            velocity: Vec2::new(speed, speed),
        })
        .add_component(Collider {
            center: Vec2::new(x_pos, y_pos),
            radius: ENEMY_SIZE,
        })
}

pub fn create_enemy_spawner(manager: &mut EntityManager) -> &Entity {
    manager.add_tag(Tag::Spawner).add_component(Spawner {
        max: MAX_ENEMY_SPAWN,
        interval: ENEMY_SPAWN_INTERVAL,
        last_spawned_duration: Duration::from_secs(0),
    })
}

pub fn create_bullet_spawner(manager: &mut EntityManager) -> &Entity {
    manager.add_tag(Tag::Bullet).add_component(Spawner {
        max: usize::MAX,
        interval: BULLET_SPAWN_INTERVAL,
        last_spawned_duration: Duration::from_secs(0),
    })
}

pub fn create_score_board(manager: &mut EntityManager) -> &Entity {
    manager.add_tag(Tag::Ui)
        .add_component(Scoreboard { current_score: 0 })
}