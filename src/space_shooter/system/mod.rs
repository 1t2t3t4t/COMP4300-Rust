use crate::{common::GameTransform, space_shooter::system::collision::BoundAxis};
use ecs::entity::EntityId;

pub mod collision;
pub mod game;
pub mod movement;
pub mod render;

pub struct EnemyKilled(pub GameTransform);

pub struct BoundCollide(pub EntityId, pub BoundAxis);
