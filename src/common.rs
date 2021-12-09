use std::any::{type_name, Any};

use ecs::entity::Entity;
use ggez::{GameError, GameResult};

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

pub trait TryGet {
    fn try_get_component<T: Any>(&self) -> GameResult<&T>;
    fn try_get_component_mut<T: Any>(&mut self) -> GameResult<&mut T>;
}

impl TryGet for Entity {
    fn try_get_component<T: Any>(&self) -> GameResult<&T> {
        self.get_component::<T>().ok_or_else(|| {
            GameError::CustomError(format!(
                "Component with type {} does not exist",
                type_name::<T>()
            ))
        })
    }

    fn try_get_component_mut<T: Any>(&mut self) -> GameResult<&mut T> {
        self.get_component_mut::<T>().ok_or_else(|| {
            GameError::CustomError(format!(
                "Component with type {} does not exist",
                type_name::<T>()
            ))
        })
    }
}
