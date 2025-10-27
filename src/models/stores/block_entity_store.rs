use std::rc::Rc;
use crate::models::block_entity::prelude::*;
use crate::models::positions::whole_position::Position;

#[derive(Debug)]
pub struct BlockEntityStore {
    block_entities: Vec<Rc<BlockEntity>>, // is this fastest even if theres a chunk full of chests?
}

impl BlockEntityStore {
    pub fn new() -> BlockEntityStore {
        BlockEntityStore { block_entities: Vec::new() }
    }
    pub fn with_capacity(capacity: usize) -> BlockEntityStore {
        BlockEntityStore { block_entities: Vec::with_capacity(capacity) }
    }

    pub fn get_at_world_position(&self, pos: &Position) -> Option<Rc<BlockEntity>> {
        for block_entity in self.block_entities.iter() {
            if block_entity.base().position() == pos {
                return Some(block_entity.clone())
            }
        }
        None
    }

    pub fn get_all(&self) -> &Vec<Rc<BlockEntity>> { &self.block_entities }

    pub fn set_at_world_position(&mut self, ent: BlockEntity) {
        for (ind, block_entity) in self.block_entities.iter().enumerate() {
            if block_entity.base().position() == ent.base().position() {
                self.block_entities.insert(ind, Rc::new(ent));
                return;
            }
        }
        self.block_entities.push(Rc::new(ent));
    }

    pub unsafe fn add_unchecked(&mut self, ent: BlockEntity) {
        self.block_entities.push(Rc::new(ent));
    }
}
