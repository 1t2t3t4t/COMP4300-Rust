use crate::{space_shooter::system::collision::BoundAxis, common::GameTransform};
use ecs::entity::EntityId;

pub mod collision;
pub mod game;
pub mod movement;
pub mod render;

pub struct EnemyKilled(pub GameTransform);

pub struct BoundCollide(pub EntityId, pub BoundAxis);
