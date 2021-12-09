use ecs::manager::EntityManager;
use ggez::{Context, GameResult};

use crate::common::TryGet;
use crate::space_shooter::component::constant::PLAYER_SPEED;
use crate::space_shooter::component::movement::Speed;
use crate::{common::Transform, math::Vec2, space_shooter::Tag};

pub fn player_movement_system(manager: &mut EntityManager, ctx: &mut Context) -> GameResult<()> {
    let players = manager.get_entities(Tag::Player);
    for player in players {
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
        let transform = player.try_get_component_mut::<Transform>()?;
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
        let transform = enemy.try_get_component_mut::<Transform>()?;
        transform.position = transform.position + (speed * dt.as_secs_f32());
    }
    Ok(())
}
