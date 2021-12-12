use crate::common::{Transform, TryGet};
use crate::space_shooter::component;
use crate::space_shooter::component::game::Spawner;
use crate::space_shooter::Tag;
use ecs::manager::EntityManager;
use ggez::{Context, GameResult};
use std::time::Duration;
use ggez::event::MouseButton;
use crate::math::Vec2;
use crate::space_shooter::component::constant::BULLET_SPEED;
use crate::space_shooter::component::create_bullet;
use crate::space_shooter::component::movement::Speed;

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
    if ggez::input::mouse::button_pressed(ctx, MouseButton::Left) {
        if let Some(player) = manager.get_entities(Tag::Player).first_mut() {
            let mouse_pos: Vec2 = ggez::input::mouse::position(ctx).into();
            let player_pos = player.try_get_component::<Transform>()?.position;
            let shoot_dir = mouse_pos - player_pos;
            let velocity = shoot_dir.normalized() * BULLET_SPEED;
            let transform = Transform {
                position: player_pos,
                rotation: Vec2::zero()
            };
            create_bullet(manager, Speed { velocity }, transform);
        }
    }
    Ok(())
}