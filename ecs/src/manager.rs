use std::any::{Any, TypeId};
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::entity::{Entity, EntityId};
use crate::type_query::TypesQueryable;

#[derive(Default)]
pub struct EntityManager<Tag = String>
where
    Tag: Hash + Eq,
{
    entities: HashMap<EntityId, Entity<Tag>>,
    component_index: HashMap<Vec<TypeId>, HashSet<EntityId>>,
    tags: HashMap<Tag, HashSet<EntityId>>,
    pending_add: HashMap<EntityId, Entity<Tag>>,
    size: u64,
}

impl<Tag> EntityManager<Tag>
where
    Tag: Hash + Eq + Default + Copy,
{
    pub fn add(&mut self) -> &mut Entity<Tag> {
        self.add_tag(Tag::default())
    }
}

impl<Tag> EntityManager<Tag>
where
    Tag: Hash + Eq + Copy,
{
    pub fn new() -> Self {
        Self {
            tags: Default::default(),
            entities: Default::default(),
            pending_add: Default::default(),
            size: Default::default(),
            component_index: Default::default()
        }
    }

    pub fn add_tag(&mut self, tag: Tag) -> &mut Entity<Tag> {
        let entity = Entity::new(self.size, tag);
        self.size += 1;
        let id = entity.id;
        self.pending_add.insert(id, entity);
        self.pending_add.get_mut(&id).unwrap()
    }

    pub fn update(&mut self) {
        self.safe_remove_entity();
        self.safe_insert_entity();
    }

    pub fn get_all(&mut self) -> Vec<&mut Entity<Tag>> {
        self.entities.values_mut().collect()
    }

    pub fn get_entity(&mut self, id: EntityId) -> Option<&mut Entity<Tag>> {
        self.entities.get_mut(&id)
    }

    pub fn get_entities_with_tag(&mut self, tag: Tag) -> Vec<&Entity<Tag>> {
        let ids = self.tags.get(&tag);
        if let Some(ids) = ids {
            ids.into_iter()
                .filter_map(|id| self.entities.get(id))
                .collect()
        } else {
            vec![]
        }
    }

    fn iter_entities_with_tag_mut(&mut self, tag: Tag) -> impl Iterator<Item = &mut Entity<Tag>> {
        let ids = self.tags.get(&tag);
        let id_set = if let Some(ids) = ids {
            HashSet::from_iter(ids.iter())
        } else {
            HashSet::new()
        };
        self.entities.iter_mut().filter_map(
            move |(id, e)| {
                if id_set.contains(id) {
                    Some(e)
                } else {
                    None
                }
            },
        )
    }

    pub fn get_entities_with_tag_mut(&mut self, tag: Tag) -> Vec<&mut Entity<Tag>> {
        self.iter_entities_with_tag_mut(tag).collect()
    }

    pub fn query_entities_component<T: Any>(&self) -> Vec<&T> {
        self.entities
            .values()
            .filter_map(|e| e.get_component::<T>())
            .collect()
    }

    pub fn query_entities_components_tag<'e, T: TypesQueryable<'e>>(
        &'e mut self,
        tag: Tag,
    ) -> Vec<T::QueryResult> {
        let mut types = T::get_types();
        types.sort();
        let component_entities_id = self.component_index.get(&types);
        let tags_entities_id = self.tags.get(&tag);
        if component_entities_id.is_none() && tags_entities_id.is_none() {
            vec![]
        } else {
            let component_entities_id = component_entities_id.unwrap();
            let tags_entities_id = tags_entities_id.unwrap();
            let (base_looping, haystack) = if component_entities_id.len() >= tags_entities_id.len() {
                (tags_entities_id, component_entities_id)
            } else {
                (component_entities_id, tags_entities_id)
            };
            base_looping
                .iter()
                .filter_map(|id| {
                    if haystack.contains(id) {
                        self.entities.get(id).and_then(|e| e.get_components::<T>())
                    } else {
                        None
                    }
                })
                .collect()
        }
    }

    pub fn query_entities_components<'e, T: TypesQueryable<'e>>(&'e self) -> Vec<T::QueryResult> {
        let mut types = T::get_types();
        types.sort();
        let entities_id = self.component_index.get(&types);
        if let Some(entities_id) = entities_id {
            entities_id
                .iter()
                .filter_map(|id| self.entities.get(id))
                .filter_map(|e| e.get_components::<T>())
                .collect()
        } else {
            vec![]
        }
    }

    pub fn query_entities_component_tag_mut<T: Any>(&mut self, tag: Tag) -> Vec<&mut T> {
        self.iter_entities_with_tag_mut(tag)
            .filter_map(|e| e.get_component_mut::<T>())
            .collect()
    }

    pub fn query_entities_component_mut<T: Any>(&mut self) -> Vec<(EntityId, &mut T)> {
        self.entities
            .values_mut()
            .filter_map(|e| {
                let id = e.id;
                match e.get_component_mut::<T>() {
                    Some(component) => Some((id, component)),
                    _ => None,
                }
            })
            .collect()
    }

    fn safe_remove_entity(&mut self) {
        let to_delete_entities: Vec<(EntityId, Tag)> = self
            .entities
            .values()
            .filter_map(|e| {
                if !e.is_alive() {
                    Some((e.id, e.tag))
                } else {
                    None
                }
            })
            .collect();

        for to_delete in to_delete_entities {
            if let Some(entities_vec) = self.tags.get_mut(&to_delete.1) {
                entities_vec.remove(&to_delete.0);
            }
            self.entities.remove(&to_delete.0);
        }
    }

    fn safe_insert_entity(&mut self) {
        let keys: Vec<EntityId> = self.pending_add.keys().copied().collect();
        for key in keys {
            let entity = self.pending_add.remove(&key).unwrap();
            let tag_entities = get_or_insert(&entity.tag, &mut self.tags);
            let component_combinations = entity.get_components_combination();
            tag_entities.insert(entity.id);
            self.entities.insert(entity.id, entity);
            for combination in component_combinations {
                let component_entities = get_or_insert(combination, &mut self.component_index);
                component_entities.insert(key);
            }
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
    use std::collections::HashMap;

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

    #[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
    enum MyTag {
        A,
        B,
        Def,
    }

    impl Default for MyTag {
        fn default() -> Self {
            Self::Def
        }
    }

    #[test]
    fn test_query_components() {
        let mut manager = EntityManager::<MyTag>::default();
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
        let mut manager = EntityManager::<MyTag>::default();
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
        let mut manager = EntityManager::<MyTag>::default();

        let id = manager.add()
            .add_component(CompA)
            .add_component(CompB)
            .id;
        manager.update();

        assert!(manager.entities.contains_key(&id));
        assert!(manager.tags.contains_key(&MyTag::default()));
        assert!(manager.tags[&MyTag::default()].contains(&id));

        assert_eq!(manager.component_index.len(), 3);
    }

    #[test]
    fn test_insert_entity_tag() {
        let mut manager = EntityManager::<MyTag>::default();
        let tag = MyTag::B;
        let id = manager.add_tag(tag).id;

        manager.update();

        assert!(manager.entities.contains_key(&id));
        assert!(manager.tags.contains_key(&tag));
        assert!(manager.tags[&tag].contains(&id));

        let id2 = manager.add_tag(tag).id;

        manager.update();

        assert!(manager.entities.contains_key(&id2));
        assert!(manager.tags.contains_key(&tag));
        assert!(manager.tags[&tag].contains(&id2));
    }

    #[test]
    fn test_get_entity() {
        let mut manager = EntityManager::<MyTag>::default();
        let id = manager.add().id;
        manager.add();
        manager.add();
        manager.update();

        let entity = manager.get_entity(id);

        assert!(entity.is_some());
        let entity = entity.unwrap();
        assert_eq!(entity.id, id);
        assert_eq!(entity.tag, MyTag::default());
    }

    #[test]
    fn test_get_entity_tag() {
        let mut manager = EntityManager::<MyTag>::default();
        let tag = MyTag::B;
        let id1 = manager.add_tag(tag).id;
        let id2 = manager.add_tag(tag).id;
        manager.add();
        manager.update();

        let entities = manager.get_entities_with_tag(tag);

        assert_eq!(entities.len(), 2);
        assert!(entities.iter().any(|e| e.id == id1));
        assert!(entities.iter().any(|e| e.id == id2));
    }

    #[test]
    fn test_remove_dead() {
        let mut manager = EntityManager::<MyTag>::default();
        let tag = MyTag::A;
        let id1 = manager.add_tag(tag).id;
        let id2 = manager.add_tag(tag).id;
        manager.add();
        manager.update();

        manager.get_entity(id1).unwrap().destroy();
        manager.update();

        assert!(manager.get_entity(id2).is_some());
        assert!(manager.get_entity(id1).is_none());

        let tag_entries = manager.get_entities_with_tag(tag);
        assert!(tag_entries.iter().any(|e| e.id == id2));
        assert!(!tag_entries.iter().any(|e| e.id == id1));
    }

    #[test]
    fn test_remove_dead_multiple_tags() {
        let mut manager = EntityManager::<MyTag>::default();
        let tag = MyTag::A;
        let id1 = manager.add_tag(tag).id;
        let id2 = manager.add().id;
        manager.add();
        manager.update();

        manager.get_entity(id1).unwrap().destroy();
        manager.update();

        assert!(manager.get_entity(id2).is_some());
        assert!(manager.get_entity(id1).is_none());

        assert!(manager.tags[&MyTag::default()].iter().any(|id| *id == id2));
        assert!(!manager.tags[&tag].iter().any(|id| *id == id1));
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
