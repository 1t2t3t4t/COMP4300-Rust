use crate::space_shooter::component::game::Scoreboard;
use crate::space_shooter::component::physics::Collider;
use crate::space_shooter::system::BoundCollide;
use crate::space_shooter::{component, Tag};
use crate::{WINDOWS_HEIGHT, WINDOWS_WIDTH};
use common::event::EventSender;
use common::game_transform::{GameTransform, TryGet};
use common::math::collision::BoxCollision;
use ecs::manager::EntityManager;
use ggez::GameResult;

pub enum BoundAxis {
    X,
    Y,
}

pub fn windows_bound_collision_system<E: EventSender<BoundCollide>>(
    manager: &mut EntityManager<Tag>,
    event_system: &mut E,
) -> GameResult<()> {
    let enemies = manager.get_entities_with_tag_mut(Tag::Enemy);
    for enemy in enemies {
        let collider = enemy.try_get_component::<Collider>()?;

        let detect = if collider.center.x - collider.radius <= 0f32 {
            Some((BoundAxis::X, 0f32 + collider.radius))
        } else if collider.center.x + collider.radius >= WINDOWS_WIDTH {
            Some((BoundAxis::X, WINDOWS_WIDTH - collider.radius))
        } else if collider.center.y - collider.radius <= 0f32 {
            Some((BoundAxis::Y, 0f32 + collider.radius))
        } else if collider.center.y + collider.radius >= WINDOWS_HEIGHT {
            Some((BoundAxis::Y, 0f32 + WINDOWS_HEIGHT - collider.radius))
        } else {
            None
        };

        if let Some((bound, reset_pos)) = detect {
            let position = &mut enemy.try_get_component_mut::<GameTransform>()?.position;
            match bound {
                BoundAxis::X => position.x = reset_pos,
                BoundAxis::Y => position.y = reset_pos,
            }
            event_system.send(BoundCollide(enemy.id, bound));
        }
    }
    Ok(())
}

pub fn player_collision_system(manager: &mut EntityManager<Tag>) -> GameResult<()> {
    const DEATH_PENALTY: i32 = 500;

    let players = manager.get_entities_with_tag(Tag::Player);
    let player = players.first().unwrap();
    let &collider = player.try_get_component::<Collider>()?;
    let enemies = manager.get_entities_with_tag_mut(Tag::Enemy);
    let mut collided = false;

    for enemy in enemies {
        if let Some(&enemy_collider) = enemy.get_component::<Collider>() {
            let enemy_collision: BoxCollision = enemy_collider.into();
            if enemy_collision.collide_aabb(&collider.into()) {
                collided = true;
                enemy.destroy();
                break;
            }
        }
    }

    if collided {
        let mut players = manager.get_entities_with_tag_mut(Tag::Player);
        players.first_mut().unwrap().destroy();
        component::create_player(manager);

        let mut scoreboard = manager.query_entities_component_mut::<Scoreboard>();
        scoreboard.first_mut().unwrap().1.current_score -= DEATH_PENALTY;
    }

    Ok(())
}
