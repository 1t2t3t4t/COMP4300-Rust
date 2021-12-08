use ecs::manager::EntityManager;
use ggez::{Context, GameError, GameResult};

use crate::{common::Transform, math::Vec2, space_shooter::Tag};
use crate::common::TryGet;
use crate::space_shooter::component::movement::Speed;

const PLAYER_SPEED: f32 = 150f32;

pub fn player_movement_system(manager: &mut EntityManager, ctx: &mut Context) -> GameResult<()> {
    let player_transforms = manager.query_entities_tag_mut::<Transform, _>(Tag::Player);
    let dt = ggez::timer::delta(ctx);
    let mut dir = Vec2::zero();
    if ggez::input::keyboard::is_key_pressed(ctx, ggez::event::KeyCode::W) {
        dir.y += -1f32;
    }
    if ggez::input::keyboard::is_key_pressed(ctx, ggez::event::KeyCode::S) {
        dir.y += 1f32;
    }
    if ggez::input::keyboard::is_key_pressed(ctx, ggez::event::KeyCode::A) {
        dir.x += -1f32;
    }
    if ggez::input::keyboard::is_key_pressed(ctx, ggez::event::KeyCode::D) {
        dir.x += 1f32;
    }
    for transform in player_transforms {
        transform.position =
            transform.position + (dir.normalized() * PLAYER_SPEED * dt.as_secs_f32());
    }

    Ok(())
}

pub fn enemy_movement_system(manager: &mut EntityManager, ctx: &mut Context) -> GameResult<()> {
    let enemies = manager.get_entities(Tag::Enemy);
    let dt = ggez::timer::delta(ctx);
    for enemy in enemies {
        let speed = enemy.try_get_component::<Speed>()?.velocity;
        let transform = enemy.try_get_component::<Transform>()?;
        transform.position =
            transform.position + (speed * dt.as_secs_f32());
    }
    Ok(())
}
