use crate::models::other::position::Position;
use crate::models::world::block::Block;

// TODO: Full block
// TODO: Filters

pub trait BlockReader {
    fn blocks(&self) -> impl Iterator<Item = &Block>;
    fn block_count(&self) -> usize;
    fn block_at_relative_position(&self, position: Position) -> Option<&Block>;
    fn block_at_relative_index(&self, index: usize) -> Option<&Block>;
}

pub trait BlockWriter {
    fn set_block_at_relative_position(&mut self, position: Position) -> bool;
    fn set_block_at_relative_index(&mut self, index: usize) -> bool;
}

pub trait BlockAccess: BlockReader + BlockWriter {}