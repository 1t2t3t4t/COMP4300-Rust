use crate::common::event::EventSender;
use crate::common::{GameTransform, TryGet};
use crate::space_shooter::component::physics::Collider;
use crate::space_shooter::system::BoundCollide;
use crate::space_shooter::Tag;
use crate::{WINDOWS_HEIGHT, WINDOWS_WIDTH};
use ecs::manager::EntityManager;
use ggez::GameResult;

pub enum BoundAxis {
    X,
    Y,
}

pub fn bound_collision_system<E: EventSender<BoundCollide>>(
    manager: &mut EntityManager,
    event_system: &mut E,
) -> GameResult<()> {
    let enemies = manager.get_entities(Tag::Enemy);
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
