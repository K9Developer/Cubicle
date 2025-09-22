use crate::models::block_entity::prelude::*;
use crate::models::positions::whole_position::Position;

#[derive(Debug)]
pub struct BlockEntityStore {
    block_entities: Vec<BlockEntity>, // is this fastest even if theres a chunk full of chests?
}

impl BlockEntityStore {
    pub fn new() -> BlockEntityStore {
        BlockEntityStore { block_entities: Vec::new() }
    }
    pub fn with_capacity(capacity: usize) -> BlockEntityStore {
        BlockEntityStore { block_entities: Vec::with_capacity(capacity) }
    }

    pub fn get_at_world_position(&self, pos: &Position) -> Option<&BlockEntity> {
        for block_entity in &self.block_entities {
            if block_entity.base().position() == pos {
                return Some(block_entity)
            }
        }
        None
    }

    pub fn get_all(&self) -> &Vec<BlockEntity> { &self.block_entities }

    pub fn set_at_world_position(&mut self, pos: &Position, ent: BlockEntity) {
        for (ind, block_entity) in self.block_entities.iter().enumerate() {
            if block_entity.base().position() == pos {
                self.block_entities.insert(ind, ent);
                return;
            }
        }
        self.block_entities.push(ent);
    }
}
