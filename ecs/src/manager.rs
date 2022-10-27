use hashbrown::{HashMap, HashSet};
use std::any::{Any, TypeId};
use std::borrow::Borrow;
use std::hash::Hash;

use crate::entity::{Entity, EntityId};
use crate::type_query::TypesQueryable;

#[derive(Default)]
pub struct EntityManager {
    entities: HashMap<EntityId, Entity>,
    component_index: HashMap<Vec<TypeId>, HashSet<EntityId>>,
    pending_add: HashMap<EntityId, Entity>,
    size: u64,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: Default::default(),
            pending_add: Default::default(),
            size: Default::default(),
            component_index: Default::default(),
        }
    }

    pub fn add(&mut self) -> &mut Entity {
        let entity = Entity::new(self.size);
        self.size += 1;
        let id = entity.id;
        self.pending_add.insert(id, entity);
        self.pending_add.get_mut(&id).unwrap()
    }

    pub fn add_tag<T: Any>(&mut self, tag: T) -> &mut Entity {
        self.add().add_component(tag)
    }

    pub fn update(&mut self) {
        self.safe_remove_entity();
        self.safe_insert_entity();
    }

    pub fn get_all(&mut self) -> Vec<&mut Entity> {
        self.entities.values_mut().collect()
    }

    pub fn get_entity(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    pub fn get_entities_with_tag<T: Any>(&mut self) -> Vec<&Entity> {
        let ids = self.component_index.get(&vec![TypeId::of::<T>()]);
        if let Some(ids) = ids {
            ids.into_iter()
                .filter_map(|id| self.entities.get(id))
                .collect()
        } else {
            vec![]
        }
    }

    pub fn get_entities_with_tag_mut<T: Any>(&mut self) -> Vec<&mut Entity> {
        let ids = self.component_index.get(&vec![TypeId::of::<T>()]);
        if let Some(ids) = ids {
            self.entities
                .iter_mut()
                .filter_map(|e| ids.contains(e.0).then_some(e.1))
                .collect()
        } else {
            vec![]
        }
    }

    pub fn query_entities_component<T: Any>(&self) -> Vec<&T> {
        let entities_id = self.component_index.get(&vec![TypeId::of::<T>()]);
        if let Some(entities_id) = entities_id {
            entities_id
                .iter()
                .filter_map(|id| self.entities.get(id).and_then(|e| e.get_component::<T>()))
                .collect()
        } else {
            vec![]
        }
    }

    pub fn query_entities_components<'e, T: TypesQueryable<'e>>(&'e self) -> Vec<T::QueryResult> {
        let entities_id = self.component_index.get(&T::get_types());
        if let Some(entities_id) = entities_id {
            entities_id
                .iter()
                .filter_map(|id| self.entities.get(id).and_then(|e| e.get_components::<T>()))
                .collect()
        } else {
            vec![]
        }
    }

    pub fn query_entities_component_tag_mut<T: Any, Tag: Any>(&mut self) -> Vec<&mut T> {
        let types = <(T, Tag) as TypesQueryable>::get_types();
        let entities_id = self.component_index.get(&types).cloned();
        if let Some(entities_id) = entities_id {
            self.entities
                .iter_mut()
                .filter_map(|(id, e)| {
                    if entities_id.contains(id) {
                        e.get_component_mut::<T>()
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            vec![]
        }
    }

    pub fn query_entities_component_mut<T: Any>(&mut self) -> Vec<(EntityId, &mut T)> {
        let entities_id = self.component_index.get(&vec![TypeId::of::<T>()]);
        if let Some(entities_id) = entities_id {
            self.entities
                .iter_mut()
                .filter_map(|e| {
                    if entities_id.contains(e.0) {
                        e.1.get_component_mut::<T>()
                            .and_then(|component| Some((e.0.clone(), component)))
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            vec![]
        }
    }

    fn safe_remove_entity(&mut self) {
        let to_delete_entities = self
            .entities
            .values()
            .filter_map(|e| if !e.is_alive() { Some(e.id) } else { None })
            .collect::<Vec<EntityId>>();

        for to_delete in to_delete_entities {
            self.entities.remove(&to_delete);
            self.component_index.values_mut().for_each(|entities| {
                entities.remove(&to_delete);
            })
        }
    }

    fn safe_insert_entity(&mut self) {
        let keys: Vec<EntityId> = self.pending_add.keys().copied().collect();
        for key in keys {
            let entity = self.pending_add.remove(&key).unwrap();
            let component_combinations = entity.get_components_combination();
            for combination in component_combinations {
                let component_entities = get_or_insert(combination, &mut self.component_index);
                component_entities.insert(key);
            }
            self.entities.insert(entity.id, entity);
        }
    }
}

fn get_or_insert<K, V, BK>(key: BK, map: &mut HashMap<K, V>) -> &mut V
where
    K: Eq + Hash + Clone,
    BK: Borrow<K>,
    V: Default,
{
    let b_key = key.borrow();
    if !map.contains_key(b_key) {
        let val = V::default();
        map.insert(b_key.clone(), val);
    }
    map.get_mut(b_key).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::manager::get_or_insert;
    use hashbrown::HashMap;

    use super::EntityManager;

    macro_rules! generate_components {
        ($a: tt) => {
            #[derive(Debug, Eq, PartialEq)]
            struct $a(String);
        };
        ($a: tt, $($b: tt),*) => {
            generate_components!($a);
            generate_components!($($b),*);
        };
    }

    generate_components!(CompA, CompB, CompC, CompD, CompE);

    struct TagA;
    struct TagB;

    #[test]
    fn test_query_components() {
        let mut manager = EntityManager::default();
        manager
            .add()
            .add_component(CompA(String::from("1")))
            .add_component(CompB(String::from("1")));
        manager
            .add()
            .add_component(CompA(String::from("2")))
            .add_component(CompB(String::from("2")))
            .add_component(CompC(String::from("2")));
        manager
            .add()
            .add_component(CompA(String::from("3")))
            .add_component(CompC(String::from("3")));
        manager.update();

        let res = manager.query_entities_components::<(CompA, CompB)>();

        assert_eq!(res.len(), 2);
        assert!(
            res.iter()
                .any(|r| *r == (&CompA(String::from("1")), &CompB(String::from("1")))),
            "Does not have 1"
        );
        assert!(
            res.iter()
                .any(|r| *r == (&CompA(String::from("2")), &CompB(String::from("2")))),
            "Does not have 2"
        );
    }

    #[test]
    fn test_query_components_mut() {
        let mut manager = EntityManager::default();
        manager
            .add()
            .add_component(CompA(String::from("1")))
            .add_component(CompB(String::from("1")));
        manager
            .add()
            .add_component(CompA(String::from("2")))
            .add_component(CompB(String::from("2")))
            .add_component(CompC(String::from("2")));
        manager
            .add()
            .add_component(CompB(String::from("3")))
            .add_component(CompC(String::from("3")));
        manager.update();

        let res = manager.query_entities_component_mut::<CompA>();

        assert_eq!(res.len(), 2);
        assert!(
            res.iter().any(|r| *(r.1) == CompA(String::from("1"))),
            "Does not have 1"
        );
        assert!(
            res.iter().any(|r| *(r.1) == CompA(String::from("2"))),
            "Does not have 2"
        );
    }

    #[test]
    fn test_insert_entity() {
        let mut manager = EntityManager::default();

        let id = manager.add().add_component(CompA).add_component(CompB).id;
        manager.update();

        assert!(manager.entities.contains_key(&id));

        assert_eq!(manager.component_index.len(), 3);
    }

    #[test]
    fn test_insert_entity_tag() {
        let mut manager = EntityManager::default();
        let id = manager.add().add_component(TagA).id;
        manager.update();

        assert!(manager.entities.contains_key(&id));

        let entity = manager.get_entities_with_tag::<TagA>();
        assert_eq!(entity.len(), 1);
        assert_eq!(entity[0].id, id);
    }

    #[test]
    fn test_get_entity() {
        let mut manager = EntityManager::default();
        let id = manager.add().id;
        manager.add();
        manager.add();
        manager.update();

        let entity = manager.get_entity(id);

        assert!(entity.is_some());
        let entity = entity.unwrap();
        assert_eq!(entity.id, id);
    }

    #[test]
    fn test_get_entity_tag() {
        let mut manager = EntityManager::default();
        let id1 = manager.add().add_component(TagA).id;
        let id2 = manager.add().add_component(TagA).id;
        manager.add();
        manager.update();

        let entities = manager.get_entities_with_tag::<TagA>();

        assert_eq!(entities.len(), 2);
        assert!(entities.iter().any(|e| e.id == id1));
        assert!(entities.iter().any(|e| e.id == id2));
    }

    #[test]
    fn test_remove_dead() {
        let mut manager = EntityManager::default();
        let id1 = manager.add().add_component(TagA).id;
        let id2 = manager.add().add_component(TagA).id;
        manager.add();
        manager.update();

        manager.get_entity(id1).unwrap().destroy();
        manager.update();

        assert!(manager.get_entity(id2).is_some());
        assert!(manager.get_entity(id1).is_none());

        let tag_entries = manager.get_entities_with_tag::<TagA>();
        assert!(tag_entries.iter().any(|e| e.id == id2));
        assert!(!tag_entries.iter().any(|e| e.id == id1));
    }

    #[test]
    fn test_remove_dead_multiple_tags() {
        let mut manager = EntityManager::default();
        let id1 = manager.add().add_component(TagA).id;
        let id2 = manager.add().add_component(TagB).id;
        manager.add();
        manager.update();

        manager.get_entity(id1).unwrap().destroy();
        manager.update();

        assert!(manager.get_entity(id2).is_some());
        assert!(manager.get_entity(id1).is_none());
    }

    #[test]
    fn test_get_or_insert() {
        let mut map = HashMap::<&str, i32>::from([("test2", 10i32)]);

        let no_key_val = get_or_insert("test", &mut map);
        assert_eq!(no_key_val, &mut 0i32);

        let val = get_or_insert("test2", &mut map);
        assert_eq!(val, &mut 10i32);
    }
}
