use ecs::entity::EntityId;

pub mod game;
pub mod movement;
pub mod render;

pub struct BoundCollide(pub EntityId);

pub mod collision {
    use ggez::GameResult;
    use ecs::manager::EntityManager;
    use crate::common::event::EventSender;
    use crate::common::TryGet;
    use crate::space_shooter::component::physics::Collider;
    use crate::space_shooter::system::BoundCollide;
    use crate::space_shooter::Tag;
    use crate::{WINDOWS_HEIGHT, WINDOWS_WIDTH};

    pub fn bound_collision_system<E: EventSender<BoundCollide>>(
        manager: &mut EntityManager,
        event_system: &mut E
    ) -> GameResult<()> {
        let enemies = manager.get_entities(Tag::Enemy);
        for enemy in enemies {
            let collider = enemy.try_get_component::<Collider>()?;
            if collider.center.x - collider.radius <= 0f32 ||
                collider.center.y - collider.radius <= 0f32 ||
                collider.center.x + collider.radius >= WINDOWS_WIDTH ||
                collider.center.y + collider.radius >= WINDOWS_HEIGHT {
                event_system.send(BoundCollide(enemy.id))
            }
        }
        Ok(())
    }
}
