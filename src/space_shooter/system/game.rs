use crate::common::TryGet;
use crate::space_shooter::component;
use crate::space_shooter::component::game::Spawner;
use crate::space_shooter::Tag;
use ecs::manager::EntityManager;
use ggez::{Context, GameResult};
use std::time::Duration;

pub fn enemy_spawner(manager: &mut EntityManager, ctx: &mut Context) -> GameResult<()> {
    let enemy_count = manager.get_entities(Tag::Enemy).len();

    let mut spawner = manager.get_entities(Tag::Spawner);
    let spawner = spawner.first_mut().unwrap();

    let info = spawner.try_get_component_mut::<Spawner>()?;
    let delta = ggez::timer::delta(ctx);
    info.last_spawned_duration = info.last_spawned_duration + delta;

    if enemy_count < info.max && info.last_spawned_duration >= info.interval {
        info.last_spawned_duration = Duration::from_secs(0);
        component::create_enemy(manager);
    }
    Ok(())
}