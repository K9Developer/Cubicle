use crate::models::filter::filter::Filter;
use crate::models::positions::whole_position::Position;
use crate::models::world::fulls::full_block::FullBlock;

pub trait BlockReader<'a> {
    fn blocks<F>(&mut self, callback: F) where F: FnMut(FullBlock<'a>) -> bool;
    fn block_count(&self) -> usize;
    fn block_at_position(&mut self, position: Position) -> Option<FullBlock<'a>>;
    fn find_blocks<F>(&mut self, filter: Filter, callback: F) where F: FnMut(FullBlock<'a>) -> bool;
}

pub trait BlockWriter {
    fn set_block_at_position(&mut self, block: FullBlock) -> bool;
}
