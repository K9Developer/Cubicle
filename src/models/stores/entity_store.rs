use crate::models::entity::entity::Entity;

pub type EntityStoreKey = usize;

pub struct EntityStore {
    entities: Vec<Entity>,
}

impl EntityStore {
    pub fn new() -> EntityStore {
        EntityStore::with_capacity((32*32)*2) // 1 region loaded with 2 entities per chunk - random and can be optimized
    }

    pub fn with_capacity(size: usize) -> EntityStore {
        EntityStore { entities: Vec::with_capacity(size) }
    }

    pub fn add_entity(&mut self, entity: Entity) -> EntityStoreKey {
        self.entities.push(entity);
        self.entities.len()-1
    }
    pub fn add_entities(&mut self, entities: Vec<Entity>) -> Vec<EntityStoreKey> {
        let mut keys = Vec::new();
        for entity in entities { keys.push(self.add_entity(entity)); }
        keys
    }
    pub fn get(&self, entity_id: EntityStoreKey) -> &Entity {
        match self.entities.get(entity_id) {
            Some(entity) => entity,
            None => panic!("EntityStoreKey {} does not exist", entity_id)
        }
    }
    pub fn get_mut(&mut self, entity_id: EntityStoreKey) -> &mut Entity {
        match self.entities.get_mut(entity_id) {
            Some(entity) => entity,
            None => panic!("EntityStoreKey {} does not exist", entity_id)
        }
    }
    pub fn get_all(&self) -> impl Iterator<Item=&Entity> { self.entities.iter() }

    pub fn remove(&mut self, entity_id: &EntityStoreKey) -> Entity {
        if entity_id >= &self.entities.len() { panic!("EntityStoreKey out of bounds"); }
        self.entities.swap_remove(*entity_id)
    }

    pub fn count(&self) -> usize { self.entities.len() }

    pub fn set(&mut self, entity_id: &EntityStoreKey, entity: Entity) -> bool {
        if entity_id >= &self.entities.len() { return false; }
        self.entities[*entity_id] = entity;
        true
    }
}