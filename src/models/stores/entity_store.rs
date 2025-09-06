use crate::models::entity::entity::Entity;

pub type EntityStoreKey = usize;

pub struct EntityStore {
    entities: Vec<Option<Entity>>,
    free_indices: Vec<EntityStoreKey>,
    dimension_name: String
}

impl EntityStore {
    pub fn new(dimension_name: &str) -> EntityStore {
        EntityStore {
            entities: Vec::new(),
            free_indices: Vec::new(),
            dimension_name: dimension_name.into(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) -> EntityStoreKey {
        match self.free_indices.pop() {
            Some(index) => { self.entities[index] = Some(entity); index}
            None => { self.entities.push(Some(entity)); self.entities.len()-1}
        }
    }
    pub fn add_entities(&mut self, entities: Vec<Entity>) -> Vec<EntityStoreKey> {
        let mut keys = Vec::new();
        for entity in entities {
            keys.push(self.add_entity(entity));
        }
        keys
    }
    pub fn get(&self, entity_id: EntityStoreKey) -> Option<&Entity> {
        match self.entities.get(entity_id) {
            Some(Some(entity)) => Some(entity),
            _ => None,
        }
    }
    pub fn get_mut(&mut self, entity_id: EntityStoreKey) -> Option<&mut Entity> {
        match self.entities.get_mut(entity_id) {
            Some(Some(entity)) => Some(entity),
            _ => None,
        }
    }
    pub fn get_all(&self) -> impl Iterator<Item=&Entity> { self.entities.iter().filter_map(|e| e.as_ref()) }

    pub fn remove(&mut self, entity_id: &EntityStoreKey) -> Option<Entity> {
        if entity_id >= &self.entities.len() { return None; }
        self.entities[*entity_id].take()
    }

    pub fn count(&self) -> usize { self.entities.len() }

    pub fn set(&mut self, entity_id: &EntityStoreKey, entity: Entity) -> bool {
        if entity_id >= &self.entities.len() { return false; }
        self.entities[*entity_id] = Some(entity);
        true
    }
}