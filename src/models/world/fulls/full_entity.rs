use std::fmt;
use std::fmt::Debug;
use crate::models::entity::entity::Entity;
use crate::models::stores::entity_store::EntityStoreKey;
use crate::types::{WorldType};

pub struct FullEntity<'a> {
    entity: Entity,
    world_ref: WorldType<'a>,
    dimension_id: String,
    entity_key: EntityStoreKey
}
impl<'a> FullEntity<'a> {
    pub fn new(world_ref: &WorldType<'a>, entity_key: EntityStoreKey, dimension_id: &str) -> Self {
        let entity: Entity = {
            let mut w = world_ref.lock().unwrap();
            let mut dim = w.dimension_mut(dimension_id).unwrap();
            (*dim.entity_store_mut().get(entity_key).unwrap()).clone()
        };

        Self {
            entity: entity,
            world_ref: world_ref.clone(),
            dimension_id: dimension_id.to_string(),
            entity_key
        }
    }

    pub fn entity(&self) -> &Entity { &self.entity }
    pub fn entity_mut(&mut self) -> &mut Entity { &mut self.entity }

    pub fn remove(&self) -> bool {
        let mut world = self.world_ref.lock().unwrap();
        match world.dimension_mut(self.dimension_id.as_str()) {
            Some(dim) => {
                dim.entity_store_mut().remove(&self.entity_key);
                true
            }
            None => { false }
        }
    }

    pub fn commit(&self) -> bool {
        let mut world = self.world_ref.lock().unwrap();
        match world.dimension_mut(self.dimension_id.as_str()) {
            Some(dim) => {
                dim.entity_store_mut().set(&self.entity_key, self.entity.clone())
            }
            None => { false }
        }
    }

}

impl Debug for FullEntity<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FullEntity")
        .field("entity", &self.entity).finish()
    }
}

// TODO: Add builder