use std::any::{type_name, Any};

use ecs::entity::Entity;
use ggez::{GameError, GameResult};

use crate::math::Vec2;

pub struct Transform {
    pub position: Vec2,
    pub rotation: Vec2,
}

impl Transform {
    pub const fn new(position: Vec2, rotation: Vec2) -> Self {
        Self { position, rotation }
    }
}

pub trait TryGet {
    fn try_get_component<T: Any>(&self) -> GameResult<&T>;
    fn try_get_component_mut<T: Any>(&mut self) -> GameResult<&mut T>;
}

impl TryGet for Entity {
    fn try_get_component<T: Any>(&self) -> GameResult<&T> {
        self.get_component::<T>().ok_or_else(|| {
            GameError::CustomError(format!(
                "Component with type {} does not exist",
                type_name::<T>()
            ))
        })
    }

    fn try_get_component_mut<T: Any>(&mut self) -> GameResult<&mut T> {
        self.get_component_mut::<T>().ok_or_else(|| {
            GameError::CustomError(format!(
                "Component with type {} does not exist",
                type_name::<T>()
            ))
        })
    }
}

pub mod event {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;

    pub trait EventSender<Event> {
        fn send(&mut self, event: Event);
    }

    pub trait EventReceiver<Event> {
        fn read(&mut self) -> Vec<Event>;
    }

    #[derive(Default, Debug)]
    pub struct EventSystem {
        events: HashMap<TypeId, Vec<Box<dyn Any>>>
    }

    impl<T> EventSender<T> for EventSystem where T : Any {
        fn send(&mut self, event: T) {
            let boxed = Box::new(event);
            if let Some(arr) = self.events.get_mut(&TypeId::of::<T>()) {
                arr.push(boxed);
            } else {
                self.events.insert(TypeId::of::<T>(), vec![boxed]);
            }
        }
    }

    impl<T> EventReceiver<T> for EventSystem where T : Any  {
        fn read(&mut self) -> Vec<T> {
            self.events.remove(&TypeId::of::<T>())
                .unwrap_or_default()
                .into_iter()
                .map(|e| *(e.downcast::<T>().unwrap()))
                .collect()
        }
    }

    #[cfg(test)]
    mod test {
        use crate::common::event::{EventReceiver, EventSender, EventSystem};

        #[derive(Eq, PartialEq)]
        struct MyEvent(String);

        #[test]
        fn test_add_event() {
            let mut system = EventSystem::default();
            let event = MyEvent("a".to_string());

            system.send(event);

            let read_events: Vec<MyEvent> = system.read();
            assert_eq!(read_events.len(), 1);
            assert_eq!(read_events.first().unwrap().0, "a".to_string());
        }

        #[test]
        fn test_add_event2() {
            let mut system = EventSystem::default();
            let event = MyEvent("a".to_string());
            let event2 = MyEvent("b".to_string());

            system.send(event);
            system.send(event2);

            let read_events: Vec<MyEvent> = system.read();
            assert_eq!(read_events.len(), 2);
            assert_eq!(read_events[0].0, "a".to_string());
            assert_eq!(read_events[1].0, "b".to_string());
        }
    }
}
