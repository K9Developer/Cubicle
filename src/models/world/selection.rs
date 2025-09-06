use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;
use crate::constants::versions::Version;
use crate::models::entity::entity::Entity;
use crate::models::filter::filter::Filter;
use crate::models::positions::chunk_position::ChunkPosition;
use crate::models::positions::whole_position::Position;
use crate::models::world::fulls::full_block::FullBlock;
use crate::models::world::fulls::full_entity::FullEntity;
use crate::traits::access::prelude::{BlockReader, BlockWriter, EntityReader, EntityWriter};
use crate::types::{ChunkType, WorldType};
use crate::utils::position_utils::{block_index_to_block_position, block_position_to_chunk_pos_and_block_index, world_position_to_chunk_position};

pub struct Selection<'a> {
    cached_chunks: HashMap<ChunkPosition, Option<ChunkType>>,
    world_ref: WorldType<'a>,
    version: Arc<Version>,
}

// TODO: This takes space and speed and it should just be an abstraction, i dont really like this... Figure out how to make this cheaper
impl<'a> Selection<'a> {
    pub fn new(world: WorldType<'a>) -> Self {
        let v = {
            let w = world.lock().unwrap();
            w.version()
        };
        Self {cached_chunks: HashMap::new(), world_ref: world, version: v}
    }

    fn lazy_get_chunk(&mut self, chunk_pos: ChunkPosition) -> Option<ChunkType> {
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

        let chunk_poses = self.cached_chunks.keys().cloned().collect::<Vec<_>>();
        for chunk_pos in chunk_poses {
            let chunk = match self.lazy_get_chunk(chunk_pos) {
                Some(c) => c,
                None => continue,
            };
            let actual_chunk = chunk.lock().unwrap();

            let mut block_position = block_index_to_block_position(actual_chunk.position(), 0, chunk_size, min_y);
            for b in actual_chunk.block_store().blocks() {
                if !callback(FullBlock::new_with_data(&self.world_ref, b, block_position.clone())) { return; };

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
                    return Some(FullBlock::new_with_data(&self.world_ref, local_block, block_position));
                }
                None
            }
            None => None
        }
    }

    fn find_blocks<F>(&mut self, filter: Filter, mut callback: F) where F: FnMut(FullBlock<'a>) -> bool {
        // TODO: We can do a ton of optimizations here like using indexes for faster lookup
        self.blocks(|block| {
            if filter.matches_block(&block) {
                if !callback(block) { return false; }
            }
            return true;
        });
    }
}

impl EntityReader for Selection<'_> {
    fn entities<F>(&mut self, mut callback: F) where F: FnMut(FullEntity) -> bool
    {
        let chunk_poses = self.cached_chunks.keys().cloned().collect::<Vec<_>>();
        for chunk_pos in chunk_poses {
            let dim_id = chunk_pos.dimension().to_string();
            let chunk = match self.lazy_get_chunk(chunk_pos) {
                Some(c) => c,
                None => continue,
            };
            let actual_chunk = chunk.lock().unwrap();
            for entity_key in actual_chunk.entity_keys() {
                if !callback(
                    FullEntity::new(
                        &self.world_ref,
                        entity_key.clone(),
                        dim_id.as_str(),
                    )
                ) { return; }
            }
        }
    }

    fn entity_count(&mut self) -> usize {
        let mut count = 0;
        let chunk_poses = self.cached_chunks.keys().cloned().collect::<Vec<_>>();
        for chunk_pos in chunk_poses {
            let chunk = match self.lazy_get_chunk(chunk_pos) {
                Some(c) => c,
                None => continue,
            };
            let actual_chunk = chunk.lock().unwrap();

            count += actual_chunk.entity_count();
        }
        count
    }

    fn find_entities<F>(&mut self, filter: Filter, mut callback: F) where F: FnMut(FullEntity) -> bool
    {
        self.entities(|entity| {
            if filter.matches_entity(&entity) {
                if !callback(entity) { return false; }
            }
            return true;
        });
    }
}

impl EntityWriter for Selection<'_> {
    fn set_entity_at_position(&mut self, entity: Entity) -> bool {
        let mut world = self.world_ref.lock().unwrap();
        let dim_id = entity.base().position().dimension();
        match world.dimension_mut(dim_id) {
            Some(dimension) => {
                let entity_pos = entity.base().position();
                let chunk_pos = world_position_to_chunk_position(entity_pos.x(), entity_pos.z(), self.version.data.chunk_size);
                let entity_key = dimension.entity_store_mut().add_entity(entity);
                let chunk = dimension.chunk_mut(chunk_pos);
                match chunk {
                    Some(chunk) => {
                        chunk.lock().unwrap().add_entity(entity_key);
                        true
                    },
                    None => false
                }
            },
            None => { false }
        }
    }
}

impl BlockWriter for Selection<'_> {
    fn set_block_at_position(&mut self, block: FullBlock) -> bool {
        if let Some((parent, rel_index)) = block.parent_chunk() {
            let mut ch = parent.lock().unwrap();
            ch.block_store_mut().set_block_at_index(rel_index, block.palette_block());
        }
        false
    }
}

pub struct SelectionBuilder<'a> {
    underlying: Selection<'a>,
}

impl<'a> SelectionBuilder<'a> {
    pub fn new(world: &WorldType<'a>) -> Self {
        SelectionBuilder {
            underlying: Selection::new(world.clone()),
        }
    }

    pub fn new_owned(world: WorldType<'a>) -> Self {
        SelectionBuilder {
            underlying: Selection::new(world),
        }
    }

    pub fn build(self) -> Selection<'a> {
        self.underlying
    }


    pub fn with_chunk_position(mut self, chunk_pos: ChunkPosition) -> Self {
        self.underlying.cached_chunks.insert(chunk_pos, None);
        self
    }

    pub fn with_chunk(mut self, chunk: ChunkType) -> Self {
        let pos = chunk.lock().unwrap().position().clone();
        self.underlying.cached_chunks.insert(pos, Some(chunk));
        self
    }

    pub fn without_chunk(mut self, chunk_pos: ChunkPosition) -> Self {
        self.underlying.cached_chunks.remove(&chunk_pos);
        self
    }

    pub fn all_dimension_chunks(mut self, dimension: &str) -> Self {
        {
            let world = self.underlying.world_ref.lock().unwrap();
            let dim = world.dimension(dimension);
            if dim.is_none() { panic!("Invalid dimension"); }
            for chunk_pos in dim.unwrap().chunk_positions() {
                self.underlying.cached_chunks.insert(ChunkPosition::new(chunk_pos.0, chunk_pos.1, dimension.clone()), None);
            }
        }
        self
    }

    pub fn all_chunks(mut self) -> Self {
        {
            let dims: Vec<String> = {
                let world = self.underlying.world_ref.lock().unwrap();
                world.dimensions().keys().cloned().collect()
            };

            for dim in dims {
                self = self.all_dimension_chunks(&dim);
            }
        }
        self
    }
}
