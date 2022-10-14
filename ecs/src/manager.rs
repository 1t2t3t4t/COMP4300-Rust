use std::any::Any;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

use crate::entity::{Entity, EntityId};
use crate::type_query::TypesQueryable;

#[derive(Default)]
pub struct EntityManager<Tag = String>
where
    Tag: Hash + Eq,
{
    entities: HashMap<EntityId, Entity<Tag>>,
    tags: HashMap<Tag, Vec<EntityId>>,
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
    pub fn add_tag(&mut self, tag: Tag) -> &mut Entity<Tag> {
        let entity = Entity::new(self.size, tag);
        self.size += 1;
        let id = entity.id;
        self.pending_add.insert(id, entity);
        self.pending_add.get_mut(&id).unwrap()
    }

    pub fn update(&mut self) {
        self.safe_remove_entity();
        self.safe_insert_entity()
    }

    pub fn get_all(&mut self) -> Vec<&mut Entity<Tag>> {
        self.entities.values_mut().collect()
    }

    pub fn get_entity(&mut self, id: EntityId) -> Option<&mut Entity<Tag>> {
        self.entities.get_mut(&id)
    }

    pub fn get_entities_tag(&mut self, tag: Tag) -> Vec<&mut Entity<Tag>> {
        if let Some(ids) = self.tags.get(&tag) {
            self.entities
                .iter_mut()
                .filter_map(|(id, e)| if ids.contains(id) { Some(e) } else { None })
                .collect()
        } else {
            vec![]
        }
    }

    pub fn query_entities_tag_mut<T: Any>(&mut self, tag: Tag) -> Vec<&mut T> {
        self.get_entities_tag(tag)
            .into_iter()
            .filter_map(|e| e.get_component_mut::<T>())
            .collect()
    }

    pub fn query_entities_component<T: Any>(&self) -> Vec<&T> {
        self.entities
            .values()
            .filter_map(|e| e.get_component::<T>())
            .collect()
    }

    pub fn query_entities_components<'e, T: TypesQueryable<'e>>(&'e self) -> Vec<T::QueryResult> {
        self.entities
            .values()
            .filter_map(|e| e.get_components::<T>())
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
        let to_delete_entities = self
            .entities
            .values()
            .filter_map(|e| {
                if !e.is_alive() {
                    Some((e.id, e.tag))
                } else {
                    None
                }
            })
            .collect::<Vec<(EntityId, Tag)>>();

        for to_delete in to_delete_entities {
            if let Some(entities_vec) = self.tags.get_mut(&to_delete.1) {
                if let Some(idx) = entities_vec.iter().position(|e_id| *e_id == to_delete.0) {
                    entities_vec.remove(idx);
                }
            }
            self.entities.remove(&to_delete.0);
        }
    }

    fn safe_insert_entity(&mut self) {
        let keys: Vec<EntityId> = self.pending_add.keys().copied().collect();
        for key in keys {
            let entity = self.pending_add.remove(&key).unwrap();
            let tag_entities = get_or_insert(&entity.tag, &mut self.tags);
            tag_entities.push(entity.id);
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
    use std::collections::HashMap;

    use super::EntityManager;

    #[derive(Debug, Eq, PartialEq)]
    struct CompA(String);
    #[derive(Debug, Eq, PartialEq)]
    struct CompB(String);
    #[derive(Debug, Eq, PartialEq)]
    struct CompC(String);

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
    fn test_insert_entity() {
        let mut manager = EntityManager::<MyTag>::default();

        let id = manager.add().id;
        manager.update();

        assert!(manager.entities.contains_key(&id));
        assert!(manager.tags.contains_key(&MyTag::default()));
        assert!(manager.tags[&MyTag::default()].contains(&id));
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

        let entities = manager.get_entities_tag(tag);

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

        let tag_entries = manager.get_entities_tag(tag);
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
