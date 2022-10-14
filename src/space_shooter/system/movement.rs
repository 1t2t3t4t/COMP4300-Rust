use ecs::manager::EntityManager;
use ggez::{Context, GameResult};
use std::time::Duration;

use crate::space_shooter::component::constant::PLAYER_SPEED;
use crate::space_shooter::component::game::DisplayTextEvent;
use crate::space_shooter::component::general::SpeedBoost;
use crate::space_shooter::component::movement::Speed;
use crate::space_shooter::component::physics::Collider;
use crate::space_shooter::system::collision::BoundAxis;
use crate::space_shooter::system::BoundCollide;
use crate::space_shooter::Tag;
use common::event::{EventReceiver, EventSender};
use common::game_transform::{GameTransform, TryGet};
use common::math::Vec2;

pub fn player_speed_boost_system(
    manager: &mut EntityManager<Tag>,
    ctx: &mut Context,
    event_sender: &mut impl EventSender<DisplayTextEvent>,
) -> GameResult<()> {
    let boosts = manager.query_entities_component_mut::<SpeedBoost>();
    let current_time = ggez::timer::time_since_start(ctx);
    let tap_boost = ggez::input::keyboard::is_key_pressed(ctx, ggez::event::KeyCode::LShift);
    let dt = ggez::timer::delta(ctx);

    for (_, boost) in boosts {
        let should_boost = match boost.last_boost {
            None => true,
            Some(last_boost) => current_time - last_boost >= Duration::from_secs(3),
        };

        if tap_boost && !boost.is_boosting && should_boost {
            boost.is_boosting = true;
            boost.time_left = Duration::from_millis(200);
            boost.last_boost = Some(current_time);
        } else if let Some(last_boost) = boost.last_boost {
            if tap_boost && !should_boost {
                event_sender.send(DisplayTextEvent {
                    text: format!(
                        "Boosting cool down wait for {} ms.",
                        (current_time - last_boost).as_millis()
                    ),
                    dur: Duration::from_secs(2),
                })
            }
        }

        if boost.is_boosting {
            if boost.time_left <= dt || boost.time_left.is_zero() {
                boost.is_boosting = false;
            } else {
                boost.time_left -= dt;
            }
        }
    }
    Ok(())
}

pub fn player_movement_system(
    manager: &mut EntityManager<Tag>,
    ctx: &mut Context,
) -> GameResult<()> {
    let players = manager.get_entities_tag(Tag::Player);
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
        let speed_boost = *player.try_get_component::<SpeedBoost>()?;
        let transform = player.try_get_component_mut::<GameTransform>()?;
        let speed = if speed_boost.is_boosting {
            PLAYER_SPEED * 4f32
        } else {
            PLAYER_SPEED
        };

        transform.position = transform.position + (dir.normalized() * speed * dt.as_secs_f32());
    }
    Ok(())
}

pub fn enemy_movement_system(
    manager: &mut EntityManager<Tag>,
    event: &mut impl EventReceiver<BoundCollide>,
    ctx: &mut Context,
) -> GameResult<()> {
    let enemies = manager.get_entities_tag(Tag::Enemy);
    let dt = ggez::timer::delta(ctx);
    let collide_events = event.read();

    for enemy in enemies {
        if let Some(collision) = collide_events.iter().find(|e| e.0 == enemy.id) {
            let velocity = &mut enemy.try_get_component_mut::<Speed>()?.velocity;
            match collision.1 {
                BoundAxis::X => velocity.x *= -1f32,
                BoundAxis::Y => velocity.y *= -1f32,
            }
        }

        let speed = enemy.try_get_component::<Speed>()?.velocity;
        let transform = enemy.try_get_component_mut::<GameTransform>()?;
        transform.position = transform.position + (speed * dt.as_secs_f32());
    }
    Ok(())
}

pub fn bullet_movement_system(
    manager: &mut EntityManager<Tag>,
    ctx: &mut Context,
) -> GameResult<()> {
    let bullets = manager.get_entities_tag(Tag::Bullet);
    let dt = ggez::timer::delta(ctx);
    for bullet in bullets {
        if let Some(speed) = bullet.get_component::<Speed>() {
            let speed = speed.velocity;
            let transform = bullet.try_get_component_mut::<GameTransform>()?;
            transform.position = transform.position + (speed * dt.as_secs_f32());
        }
    }
    Ok(())
}

pub fn collider_follow_transform_system(manager: &mut EntityManager<Tag>) -> GameResult<()> {
    let entities = manager.get_all();
    for entity in entities {
        if let Some(transform) = entity.get_component::<GameTransform>() {
            let updated_pos = transform.position;
            if let Some(collider) = entity.get_component_mut::<Collider>() {
                collider.center = updated_pos;
            }
        }
    }
    Ok(())
}
