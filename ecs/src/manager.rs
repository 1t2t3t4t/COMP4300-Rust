use std::{
    collections::{HashMap, VecDeque},
    marker::PhantomData,
};

use crate::entity::{Entity, EntityId};

pub struct EntityTag<T>(PhantomData<T>);

pub const DEFAULT_ENTITY_TAG: &str = "Default";

#[derive(Default)]
pub struct EntityManager {
    entities: HashMap<EntityId, Entity>,
    tags: HashMap<String, Vec<EntityId>>,
    pending_add: VecDeque<Entity>,
    size: u64,
}

impl EntityManager {
    pub fn add(&mut self) -> EntityId {
        self.add_tag(DEFAULT_ENTITY_TAG)
    }

    pub fn add_tag<S: ToString>(&mut self, tag: S) -> EntityId {
        let entity = Entity::new(self.size, tag.to_string());
        self.size += 1;
        let id = entity.id;
        self.pending_add.push_back(entity);
        id
    }

    pub fn update(&mut self) {
        self.safe_remove_entity();
        self.safe_insert_entity()
    }

    pub fn get_entity<'a>(&'a mut self, id: EntityId) -> Option<&'a mut Entity> {
        self.entities.get_mut(&id)
    }

    pub fn get_entities<'a, S: ToString>(&'a mut self, tag: S) -> Vec<&'a mut Entity> {
        if let Some(ids) = self.tags.get(&tag.to_string()) {
            self.entities
                .iter_mut()
                .filter_map(|(id, e)| if ids.contains(id) { Some(e) } else { None })
                .collect()
        } else {
            vec![]
        }
    }

    fn safe_remove_entity(&mut self) {
        let to_delete_entities = self
            .entities
            .values()
            .filter_map(|e| {
                if !e.is_alive() {
                    Some((e.id, e.tag.clone()))
                } else {
                    None
                }
            })
            .collect::<Vec<(EntityId, String)>>();
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
        while let Some(entity) = self.pending_add.pop_front() {
            if let Some(tag_entities) = self.tags.get_mut(&entity.tag) {
                tag_entities.push(entity.id)
            } else {
                self.tags.insert(entity.tag.clone(), vec![entity.id]);
            }
            self.entities.insert(entity.id, entity);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::manager::DEFAULT_ENTITY_TAG;

    use super::EntityManager;

    #[test]
    fn test_insert_entity() {
        let mut manager = EntityManager::default();

        let id = manager.add();
        manager.update();

        assert!(manager.entities.contains_key(&id));
        assert!(manager.tags.contains_key(DEFAULT_ENTITY_TAG));
        assert!(manager.tags[DEFAULT_ENTITY_TAG].contains(&id));
    }

    #[test]
    fn test_insert_entity_tag() {
        let mut manager = EntityManager::default();
        let tag = "MyTag";

        let id = manager.add_tag(tag);
        manager.update();

        assert!(manager.entities.contains_key(&id));
        assert!(manager.tags.contains_key(tag));
        assert!(manager.tags[tag].contains(&id));
    }

    #[test]
    fn test_get_entity() {
        let mut manager = EntityManager::default();
        let id = manager.add();
        manager.add();
        manager.add();
        manager.update();

        let entity = manager.get_entity(id);

        assert!(entity.is_some());
        let entity = entity.unwrap();
        assert_eq!(entity.id, id);
        assert_eq!(entity.tag, DEFAULT_ENTITY_TAG);
    }

    #[test]
    fn test_get_entity_tag() {
        let mut manager = EntityManager::default();
        let tag = "MyTag";
        let id1 = manager.add_tag(tag);
        let id2 = manager.add_tag(tag);
        manager.add();
        manager.update();

        let entities = manager.get_entities(tag);

        assert_eq!(entities.len(), 2);
        assert!(entities.iter().any(|e| e.id == id1));
        assert!(entities.iter().any(|e| e.id == id2));
    }

    #[test]
    fn test_remove_dead() {
        let mut manager = EntityManager::default();
        let tag = "MyTag";
        let id1 = manager.add_tag(tag);
        let id2 = manager.add_tag(tag);
        manager.add();
        manager.update();

        manager.get_entity(id1).unwrap().destroy();
        manager.update();

        assert!(manager.get_entity(id2).is_some());
        assert!(manager.get_entity(id1).is_none());

        let tag_entries = manager.get_entities(tag);
        assert!(tag_entries.iter().any(|e| e.id == id2));
        assert!(!tag_entries.iter().any(|e| e.id == id1));
    }
}
