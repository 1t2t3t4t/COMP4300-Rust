use std::any::{type_name, Any};

use ecs::entity::Entity;
use ecs::TypesQueryable;
use ggez::{GameError, GameResult};

use crate::math::Vec2;

#[derive(Clone)]
pub struct GameTransform {
    pub position: Vec2,
    pub rotation: Vec2,
}

impl GameTransform {
    pub const fn new(position: Vec2, rotation: Vec2) -> Self {
        Self { position, rotation }
    }
}

pub trait TryGet {
    fn try_get_component<T: Any>(&self) -> GameResult<&T>;
    fn try_get_component_mut<T: Any>(&mut self) -> GameResult<&mut T>;

    fn try_get_components<'e, T: TypesQueryable<'e>>(&'e self) -> GameResult<T::QueryResult>;
}

impl<Tag> TryGet for Entity<Tag> {
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

    fn try_get_components<'e, T: TypesQueryable<'e>>(&'e self) -> GameResult<T::QueryResult> {
        self.get_components::<T>().ok_or_else(|| {
            GameError::CustomError(format!(
                "Components with type {} does not exist",
                type_name::<T>()
            ))
        })
    }
}
