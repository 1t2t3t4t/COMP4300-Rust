use common::event::EventSender;
use common::game_transform::{GameTransform, TryGet};

use crate::space_shooter::component;
use crate::space_shooter::component::game::{Scoreboard, Spawner};
use crate::space_shooter::component::physics::Collider;
use crate::space_shooter::Tag;
use ecs::manager::EntityManager;
use ggez::graphics::{Color, DrawMode};
use ggez::{Context, GameResult};
use std::time::Duration;

use crate::space_shooter::component::constant::BULLET_SPEED;
use crate::space_shooter::component::create_bullet;
use crate::space_shooter::component::general::{Lifespan, Score};
use crate::space_shooter::component::movement::Speed;
use common::math::collision::BoxCollision;
use common::math::Vec2;
use ecs::entity::EntityId;
use ggez::event::MouseButton;

use super::EnemyKilled;

pub fn enemy_spawner(manager: &mut EntityManager, ctx: &mut Context) -> GameResult<()> {
    let enemy_count = manager.get_entities_tag(Tag::Enemy).len();

    let mut spawner = manager.get_entities_tag(Tag::Spawner);
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

        if let Some(player) = manager.get_entities_tag(Tag::Player).first_mut() {
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
    let lifespans = manager.query_entities_component_mut::<Lifespan>();
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

pub fn aim_system(manager: &mut EntityManager, ctx: &mut Context) -> GameResult<()> {
    let entity = manager.get_entities_tag(Tag::Player);
    let player = entity.first().unwrap();
    let collider = player.try_get_component::<Collider>()?;

    let mouse_pos: Vec2 = ggez::input::mouse::position(ctx).into();
    let aim_pos = Vec2::new(
        mouse_pos.x.clamp(
            collider.center.x - collider.radius,
            collider.center.x + collider.radius,
        ),
        mouse_pos.y.clamp(
            collider.center.y - collider.radius,
            collider.center.y + collider.radius,
        ),
    );

    let aim_circle = ggez::graphics::MeshBuilder::new()
        .circle(DrawMode::fill(), aim_pos, 8f32, 0.1, Color::GREEN)?
        .build(ctx)?;

    ggez::graphics::draw(ctx, &aim_circle, ([0f32, 0f32],))?;

    Ok(())
}

pub fn kill_enemy_system(
    manager: &mut EntityManager,
    sender: &mut impl EventSender<EnemyKilled>,
) -> GameResult<()> {
    let bullets = manager
        .get_entities_tag(Tag::Bullet)
        .into_iter()
        .filter_map(|b| match b.try_get_component::<Collider>() {
            Ok(&c) => Some((b.id, c)),
            _ => None,
        })
        .collect::<Vec<(EntityId, Collider)>>();

    let enemies = manager.get_entities_tag(Tag::Enemy);
    let mut bullet_to_destroy = Vec::<EntityId>::new();
    let mut sum_score = 0;

    for enemy in enemies {
        let enemy_collider: BoxCollision = enemy.try_get_component::<Collider>()?.into();
        if let Some(collide_bullet) = bullets
            .iter()
            .find(|b| enemy_collider.collide_aabb(&b.1.into()))
        {
            enemy.destroy();
            bullet_to_destroy.push(collide_bullet.0);
            sum_score += enemy.try_get_component::<Score>()?.0;
            let enemy_transform = enemy.try_get_component::<GameTransform>()?.clone();
            sender.send(EnemyKilled(enemy_transform));
        }
    }

    for id in bullet_to_destroy {
        if let Some(entity) = manager.get_entity(id) {
            entity.destroy();
        }
    }

    let mut scoreboard = manager.query_entities_component_mut::<Scoreboard>();
    scoreboard.first_mut().unwrap().1.current_score += sum_score;

    Ok(())
}
