use ecs::manager::EntityManager;
use ggez::{Context, GameResult};

use crate::common::TryGet;
use crate::space_shooter::component::constant::PLAYER_SPEED;
use crate::space_shooter::component::movement::Speed;
use crate::{common::Transform, math::Vec2, space_shooter::Tag};
use crate::common::event::EventReceiver;
use crate::space_shooter::component::physics::Collider;
use crate::space_shooter::system::BoundCollide;

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

pub fn enemy_movement_system<E: EventReceiver<BoundCollide>>(manager: &mut EntityManager, event: &mut E, ctx: &mut Context) -> GameResult<()> {
    let enemies = manager.get_entities(Tag::Enemy);
    let dt = ggez::timer::delta(ctx);
    let collide_events = event.read();

    for enemy in enemies {
        if collide_events.iter().any(|e| e.0 == enemy.id) {
            let speed = enemy.try_get_component_mut::<Speed>()?;
            speed.velocity = speed.velocity * -1f32;
        }
        let speed = enemy.try_get_component::<Speed>()?.velocity;
        let transform = enemy.try_get_component_mut::<Transform>()?;
        transform.position = transform.position + (speed * dt.as_secs_f32());
    }
    Ok(())
}

pub fn collider_follow_transform_system(manager: &mut EntityManager) -> GameResult<()> {
    let entities = manager.get_all();
    for entity in entities {
        if let Some(transform) = entity.get_component::<Transform>() {
            let updated_pos = transform.position;
            if let Some(collider) = entity.get_component_mut::<Collider>() {
                collider.center = updated_pos;
            }
        }
    }
    Ok(())
}