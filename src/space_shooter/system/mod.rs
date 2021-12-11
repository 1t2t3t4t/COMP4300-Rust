use ecs::entity::EntityId;
use crate::space_shooter::system::collision::BoundAxis;

pub mod collision;
pub mod game;
pub mod movement;
pub mod render;

pub struct BoundCollide(pub EntityId, pub BoundAxis);
