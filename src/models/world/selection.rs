use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;
use crate::constants::versions::Version;
use crate::models::filter::filter::Filter;
use crate::models::positions::chunk_position::ChunkPosition;
use crate::models::positions::whole_position::Position;
use crate::models::world::full_block::FullBlock;
use crate::traits::access::prelude::{BlockReader, BlockWriter};
use crate::types::{ChunkType, WorldType};
use crate::utils::position_utils::{block_index_to_block_position, block_position_to_chunk_pos_and_block_index};

pub struct Selection<'a> {
    cached_chunks: HashMap<ChunkPosition, Option<ChunkType<'a>>>,
    world_ref: WorldType<'a>,
    version: Arc<Version>,
}

// TODO: This takes space and speed and it should just be an abstraction, i dont really like this... Figure out how to make this cheaper
impl<'a> Selection<'a> {
    pub fn new(world: &WorldType<'a>) -> Self {
        let world = (*world).clone();
        let v = {
            let w = world.lock().unwrap();
            w.version()
        };
        Self {cached_chunks: HashMap::new(), world_ref: world, version: v}
    }

    pub fn selection_add_chunk_position(mut self, chunk_pos: ChunkPosition) -> Self {
        self.cached_chunks.insert(chunk_pos, None);
        self
    }

    pub fn selection_remove_chunk_position(&mut self, chunk_pos: ChunkPosition) -> bool {
        match self.cached_chunks.remove(&chunk_pos) {
            Some(_) => {true}
            None => {false}
        }
    }

    pub fn selection_add_chunk(&mut self, chunk: ChunkType<'a>) -> &'a mut Selection {
        let chunk_pos = {
            let ch = chunk.lock().unwrap();
            ch.position().clone()
        };
        self.cached_chunks.insert(chunk_pos, Some(chunk));
        self
    }

    fn lazy_get_chunk(&mut self, chunk_pos: ChunkPosition) -> Option<ChunkType<'a>> {
        match self.cached_chunks.entry(chunk_pos) {
            Entry::Occupied(mut occ) => {
                let chunk_pos = occ.key().clone();
                let slot = occ.get_mut();
                if slot.is_none() {
                    let mut world = self.world_ref.lock().unwrap();
                    let c = world.dimension_mut(chunk_pos.dimension()).unwrap().chunk_mut(chunk_pos.position());
                    *slot = c;
                }
                slot.clone()
            }
            Entry::Vacant(_) => { None }
        }
    }
}

impl<'a> BlockReader<'a> for Selection<'a> {

    // callback can return bool. true means continue, false means stop
    fn blocks<F>(&mut self, mut callback: F) where F: FnMut(FullBlock<'a>) -> bool,
    {
        let chunk_size = self.version.data.chunk_size;
        let min_y = self.version.data.lowest_y;

        let keys: Vec<_> = self.cached_chunks.keys().cloned().collect();
        for chunk_pos in keys {
            let mut block_position = block_index_to_block_position(&chunk_pos, 0, chunk_size, min_y);
            if let Some(chunk) = self.lazy_get_chunk(chunk_pos) {
                let lch = chunk.lock().unwrap();
                for b in lch.block_store().blocks() {
                    if !callback(FullBlock::new_with_data(self.world_ref.clone(), b, block_position.clone())) { return; };

                    // position updates, maybe faster than re-calculating, havent tested
                    block_position.set_x(block_position.x() + 1);
                    if block_position.x() == chunk_size {
                        block_position.set_x(0);
                        block_position.set_z(block_position.z() + 1);
                        if block_position.z() == chunk_size {
                            block_position.set_z(0);
                            block_position.set_y(block_position.y() + 1);
                        }
                    }
                }
            }
        }
    }

    fn block_count(&self) -> usize {
        let layer_size = (self.version.data.chunk_size * self.version.data.chunk_size) as usize;
        let layers = (self.version.data.lowest_y.abs() + self.version.data.highest_y.abs()) as usize;

        self.cached_chunks.len() * layer_size * layers
    }

    fn block_at_position(&mut self, position: Position) -> Option<FullBlock<'a>> {
        let lowest_y = self.version.data.lowest_y;
        let chunk_size = self.version.data.chunk_size;

        let (chunk_pos, relative_index) = block_position_to_chunk_pos_and_block_index(&position, chunk_size, lowest_y);
        let ch = self.lazy_get_chunk(chunk_pos);
        match ch {
            Some(ch) => {
                let lch = ch.lock().unwrap();
                if let Some(local_block) = lch.block_store().get_block_at_index(relative_index) {
                    let block_position = block_index_to_block_position(lch.position(), relative_index, chunk_size, lowest_y);
                    return Some(FullBlock::new_with_data(self.world_ref.clone(), local_block, block_position));
                }
                None
            }
            None => None
        }
    }

    // limit of 0 is infinite
    fn find_blocks(&mut self, filter: Filter, limit: usize) -> Vec<FullBlock<'a>> {
        // TODO: We can do a ton of optimizations here like using indexes for faster lookup

        let mut blocks = Vec::new();

        self.blocks(|block| {
            if filter.matches_block(&block) {
                if limit != 0 && blocks.len() >= limit { return false; }
                blocks.push(block);
            }
            return true;
        });

        blocks
    }
}

impl BlockWriter for Selection<'_> {
    fn set_block_at_relative_position(&mut self, position: Position) -> bool {
        todo!()
    }

    fn set_block_at_relative_index(&mut self, index: usize) -> bool {
        todo!()
    }
}

// TODO: Convert stuff to return FullBlock and then continue filter.rs at 28, and filter_keys.rs