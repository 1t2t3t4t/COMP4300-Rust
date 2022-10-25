use crate::type_query::TypesQueryable;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub type EntityId = u64;

#[derive(Debug)]
pub struct Entity<Tag> {
    pub id: EntityId,
    pub tag: Tag,
    alive: bool,
    components: HashMap<TypeId, Box<dyn Any>>,
    components_combination: Vec<Vec<TypeId>>,
}

impl<Tag> Entity<Tag> {
    pub(crate) fn new(id: EntityId, tag: Tag) -> Self {
        Self {
            id,
            alive: true,
            tag,
            components: Default::default(),
            components_combination: Default::default()
        }
    }

    pub fn destroy(&mut self) {
        self.alive = false;
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    fn update_combinations(&mut self) {
        let component_types: Vec<TypeId> = self.components.keys().cloned().collect();
        let mut results: Vec<Vec<TypeId>> = Vec::new();

        for i in 0..component_types.len() {
            results.push(vec![component_types[i]]);
            let mut combinations: Vec<TypeId> = vec![component_types[i]];

            for j in (i + 1)..component_types.len() {
                combinations.push(component_types[j]);
                combinations.sort();
                results.push(combinations.clone());

                if j != i + 1 {
                    let mut v = vec![component_types[i], component_types[j]];
                    v.sort();
                    results.push(v);
                }
            }
        }
        self.components_combination = results;
    }

    pub(crate) fn get_components_combination(&self) -> &Vec<Vec<TypeId>> {
        &self.components_combination
    }

    pub fn add_component<T: Any>(&mut self, component: T) -> &mut Self {
        let added = self
            .components
            .insert(TypeId::of::<T>(), Box::new(component));
        debug_assert!(added.is_none(), "Component already added");
        self.update_combinations();
        self
    }

    pub fn get_component<T: Any>(&self) -> Option<&T> {
        self.components
            .get(&TypeId::of::<T>())
            .and_then(|v| v.downcast_ref::<T>())
    }

    pub fn get_component_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .and_then(|v| v.downcast_mut::<T>())
    }

    pub fn get_components<'e, T: TypesQueryable<'e>>(&'e self) -> Option<T::QueryResult> {
        T::query(self)
    }

    pub fn has_component<T: Any>(&self) -> bool {
        self.components.contains_key(&TypeId::of::<T>())
    }

    pub fn has_components<'e, T: TypesQueryable<'e>>(&'e self) -> bool {
        T::get_types()
            .iter()
            .all(|id| self.components.contains_key(id))
    }
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;

    use super::{Entity, TypesQueryable};

    struct MyComponent;

    struct OtherComponent;

    fn get_types<'e, T: TypesQueryable<'e>>() -> Vec<TypeId> {
        T::get_types()
    }

    #[test]
    fn test_query_types() {
        let types = get_types::<(MyComponent, OtherComponent)>();

        assert_eq!(types[0], TypeId::of::<MyComponent>());
        assert_eq!(types[1], TypeId::of::<OtherComponent>());
    }

    #[test]
    fn test_components_combination() {
        struct A;
        struct B;
        struct C;

        let mut entity = Entity::new(1, "".to_string());
        entity
            .add_component(A)
            .add_component(B)
            .add_component(C);
        
        let test_contain = |v: &[Vec<TypeId>], mut types: Vec<TypeId>| -> bool {
            types.sort();
            v.contains(&types)
        };

        let results = entity.get_components_combination();
        assert_eq!(results.len(), 7);
        assert!(test_contain(&results, vec![TypeId::of::<A>()]));
        assert!(test_contain(&results, vec![TypeId::of::<B>()]));
        assert!(test_contain(&results, vec![TypeId::of::<C>()]));

        assert!(test_contain(&results, vec![TypeId::of::<A>(), TypeId::of::<B>()]));
        assert!(test_contain(&results, vec![TypeId::of::<A>(), TypeId::of::<C>()]));
        assert!(test_contain(&results, vec![TypeId::of::<B>(), TypeId::of::<C>()]));

        assert!(test_contain(&results, vec![TypeId::of::<A>(), TypeId::of::<B>(), TypeId::of::<C>()]));
    }

    #[test]
    fn test_get_component_not_exist() {
        let mut entity = Entity::new(1, "".to_string());
        let comp = MyComponent;

        entity.add_component(comp);

        let res = entity.get_component_mut::<OtherComponent>();
        assert!(res.is_none());
        assert!(!entity.has_component::<OtherComponent>())
    }

    #[test]
    fn test_get_component() {
        let mut entity = Entity::new(1, "".to_string());
        let comp = MyComponent;

        entity.add_component(comp);

        let res = entity.get_component_mut::<MyComponent>();
        assert!(res.is_some());
        assert!(entity.has_component::<MyComponent>())
    }

    #[test]
    fn test_get_components() {
        let mut entity = Entity::new(1, "".to_string());
        entity.add_component(MyComponent);
        entity.add_component(OtherComponent);

        let res = entity.get_components::<(MyComponent, OtherComponent)>();
        assert!(res.is_some());

        let res = entity.get_components::<(OtherComponent, MyComponent)>();
        assert!(res.is_some());
    }

    #[test]
    fn test_check_components() {
        struct RandomComponent;

        let mut entity = Entity::new(1, "".to_string());
        entity.add_component(MyComponent);
        entity.add_component(OtherComponent);

        assert!(entity.has_component::<MyComponent>());
        assert!(entity.has_component::<OtherComponent>());
        assert!(!entity.has_component::<RandomComponent>());
    }

    #[test]
    fn test_check_multi_components() {
        let mut entity = Entity::new(1, "".to_string());
        entity.add_component(MyComponent);
        entity.add_component(OtherComponent);

        assert!(entity.has_components::<(MyComponent, OtherComponent)>());
        assert!(entity.has_components::<(OtherComponent, MyComponent)>());
    }

    #[test]
    fn test_check_multi_components_mismatch() {
        struct RandomComponent;

        let mut entity = Entity::new(1, "".to_string());
        entity.add_component(MyComponent);
        entity.add_component(RandomComponent);

        assert!(!entity.has_components::<(MyComponent, OtherComponent)>());
        assert!(!entity.has_components::<(OtherComponent, MyComponent)>());
    }
}
