use crate::common::{GameTransform, TryGet};
use crate::space_shooter::component;
use crate::space_shooter::component::game::Spawner;
use crate::space_shooter::Tag;
use ecs::manager::EntityManager;
use ggez::{Context, GameResult};
use std::time::Duration;

use crate::math::Vec2;
use crate::space_shooter::component::constant::BULLET_SPEED;
use crate::space_shooter::component::create_bullet;
use crate::space_shooter::component::general::Lifespan;
use crate::space_shooter::component::movement::Speed;
use ecs::entity::EntityId;
use ggez::event::MouseButton;

pub fn enemy_spawner(manager: &mut EntityManager, ctx: &mut Context) -> GameResult<()> {
    let enemy_count = manager.get_entities(Tag::Enemy).len();

    let mut spawner = manager.get_entities(Tag::Spawner);
    let spawner = spawner.first_mut().unwrap();

    let info = spawner.try_get_component_mut::<Spawner>()?;
    let delta = ggez::timer::delta(ctx);
    info.last_spawned_duration += delta;

    if enemy_count < info.max && info.last_spawned_duration >= info.interval {
        info.last_spawned_duration = Duration::from_secs(0);
        component::create_enemy(manager);
    }
    Ok(())
}

pub fn shoot_system(manager: &mut EntityManager, ctx: &mut Context) -> GameResult<()> {
    let mut query = manager.query_entities_tag_mut::<Spawner, _>(Tag::Bullet);
    let spawner = query.first_mut().unwrap();
    let dt = ggez::timer::delta(ctx);
    let can_shoot = spawner.last_spawned_duration >= spawner.interval;
    spawner.last_spawned_duration += dt;

    if can_shoot && ggez::input::mouse::button_pressed(ctx, MouseButton::Left) {
        spawner.last_spawned_duration = Duration::from_secs(0);

        if let Some(player) = manager.get_entities(Tag::Player).first_mut() {
            let mouse_pos: Vec2 = ggez::input::mouse::position(ctx).into();
            let player_pos = player.try_get_component::<GameTransform>()?.position;
            let shoot_dir = mouse_pos - player_pos;
            let velocity = shoot_dir.normalized() * BULLET_SPEED;
            let transform = GameTransform {
                position: player_pos,
                rotation: Vec2::zero(),
            };
            create_bullet(manager, Speed { velocity }, transform);
        }
    }
    Ok(())
}

pub fn lifespan_system(manager: &mut EntityManager, ctx: &mut Context) -> GameResult<()> {
    let lifespans = manager.query_entities_mut::<Lifespan>();
    let dt = ggez::timer::delta(ctx);
    let mut to_kill_ids = Vec::<EntityId>::with_capacity(lifespans.len());

    for (id, life) in lifespans {
        if let Some(subtracted) = life.time_left.checked_sub(dt) {
            life.time_left = subtracted;
        } else {
            to_kill_ids.push(id);
        }
    }

    for id in to_kill_ids {
        manager.get_entity(id).unwrap().destroy();
    }

    Ok(())
}
