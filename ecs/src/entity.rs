use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub type EntityId = u64;

#[derive(Debug)]
pub struct Entity {
    pub id: EntityId,
    pub tag: String,
    alive: bool,
    components: HashMap<TypeId, Box<dyn Any>>,
}

impl Entity {
    pub(crate) fn new(id: EntityId, tag: String) -> Self {
        Self {
            id,
            alive: true,
            tag,
            components: HashMap::new(),
        }
    }

    pub fn destroy(&mut self) {
        self.alive = false;
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn add_component<T: Any>(&mut self, component: T) -> &mut Self {
        self.components
            .insert(TypeId::of::<T>(), Box::new(component));
        self
    }

    pub fn get_component<'a, T: Any>(&'a mut self) -> Option<&'a mut T> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .and_then(|v| v.downcast_mut::<T>())
    }

    pub fn has_component<T: Any>(&self) -> bool {
        self.components.contains_key(&TypeId::of::<T>())
    }
}

#[cfg(test)]
mod tests {
    use super::Entity;

    struct MyComponent;

    struct OtherComponent;

    #[test]
    fn test_get_component_not_exist() {
        let mut entity = Entity::new(1, "".to_string());
        let comp = MyComponent;

        entity.add_component(comp);

        let res = entity.get_component::<OtherComponent>();
        assert!(res.is_none());
        assert!(!entity.has_component::<OtherComponent>())
    }

    #[test]
    fn test_get_component() {
        let mut entity = Entity::new(1, "".to_string());
        let comp = MyComponent;

        entity.add_component(comp);

        let res = entity.get_component::<MyComponent>();
        assert!(res.is_some());
        assert!(entity.has_component::<MyComponent>())
    }
}
