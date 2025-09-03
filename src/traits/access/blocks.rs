use crate::models::filter::filter::Filter;
use crate::models::positions::whole_position::Position;
use crate::models::world::full_block::FullBlock;
// TODO: Full block
// TODO: Filters

pub trait BlockReader<'a> {
    fn blocks<F>(&mut self, callback: F) where F: FnMut(FullBlock<'a>) -> bool;
    fn block_count(&self) -> usize;
    fn block_at_position(&mut self, position: Position) -> Option<FullBlock<'a>>;
    fn find_blocks(&mut self, filter: Filter, limit: usize) -> Vec<FullBlock<'a>>;
}

pub trait BlockWriter {
    fn set_block_at_relative_position(&mut self, position: Position) -> bool;
    fn set_block_at_relative_index(&mut self, index: usize) -> bool;
}

pub trait BlockAccess<'a>: BlockReader<'a> + BlockWriter {}