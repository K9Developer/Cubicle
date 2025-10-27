use std::cell::RefCell;
use std::rc::Rc;
use crate::models::block_entity::prelude::BlockEntity;
use crate::models::world::fulls::block_states::BlockStates;

pub struct BlockData {
    states: Rc<RefCell<BlockStates>>,
    block_entity: Option<Rc<BlockEntity>>,
}

impl BlockData {
    pub fn new(states: BlockStates, block_entity: Option<Rc<BlockEntity>>) -> Self {
        BlockData {
            states: Rc::new(RefCell::new(states)), block_entity
        }
    }

    pub fn states(&self) -> Rc<RefCell<BlockStates>> { self.states.clone() }
    pub fn set_states(&mut self, states: BlockStates) { self.states = Rc::new(RefCell::new(states)) }
    pub fn data(&self) -> Option<Rc<BlockEntity>> { self.block_entity.clone() }
}

