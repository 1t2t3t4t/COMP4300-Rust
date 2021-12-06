use std::{collections::{HashMap, VecDeque}, marker::PhantomData};

use crate::entity::{EntityId, Entity};

pub struct EntityTag<T>(PhantomData<T>);

pub const DEFAULT_ENTITY_TAG: &str = "Default";

#[derive(Default)]
pub struct EntityManager {
    entities: HashMap<EntityId, Entity>,
    tags: HashMap<String, Vec<EntityId>>,
    pending_add: VecDeque<Entity>,
    size: u64
}

impl EntityManager {
    pub fn add(&mut self) -> EntityId {
        self.add_tag(DEFAULT_ENTITY_TAG)
    }

    pub fn add_tag(&mut self, tag: &str) -> EntityId {
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

    pub fn get_entity(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    pub fn get_entities(&mut self, tag: &str) -> Vec<&mut Entity> {
        if let Some(ids) = self.tags.get(tag) {
            self.entities.iter_mut().filter_map(|(id, e)| {
                if ids.contains(id) {
                    Some(e)
                } else {
                    None
                }
            }).collect()
        } else {
            vec![]
        }
    }

    fn safe_remove_entity(&mut self) {
        let to_delete_entities = self.entities.values().filter(|e| !e.is_alive()).collect::<Vec<&Entity>>();
        for to_delete in to_delete_entities {
            if let Some(entities_vec) = self.tags.get_mut(&to_delete.tag) {
                if let Some(idx) = entities_vec.iter().position(|e_id| *e_id == to_delete.id) {
                    entities_vec.remove(idx);
                }
            }
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