use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::{Arc, MutexGuard};
use crate::constants::versions::Version;
use crate::models::entity::entity::Entity;
use crate::models::filter::filter::Filter;
use crate::models::positions::chunk_position::ChunkPosition;
use crate::models::positions::whole_position::Position;
use crate::models::world::fulls::full_block::FullBlock;
use crate::models::world::fulls::full_entity::FullEntity;
use crate::models::world::world::World;
use crate::traits::access::prelude::{BlockReader, BlockWriter, EntityReader, EntityWriter};
use crate::types::{ChunkType, WorldType};
use crate::utils::position_utils::{block_index_to_block_position, block_position_to_chunk_pos_and_block_index, chunk_position_to_world_position, world_position_to_chunk_position};

pub struct Selection<'r, 'a> {
    cached_chunks: HashMap<ChunkPosition, Option<ChunkType>>,
    world_ref: &'r mut World<'a>,
    version: Arc<Version>,
}

// TODO: This takes space and speed and it should just be an abstraction, i dont really like this... Figure out how to make this cheaper
impl<'r, 'a> Selection<'r, 'a> {
    pub fn new(world: &'r mut World<'a>, version: Arc<Version>) -> Self {
        Self {cached_chunks: HashMap::new(), world_ref: world, version}
    }

    pub fn chunk(&mut self, chunk_position: ChunkPosition) -> Option<ChunkType> {
        self.lazy_get_chunk(chunk_position)
    }

    pub fn chunk_count(&self) -> usize {
        self.cached_chunks.len()
    }

    fn lazy_get_chunk(&mut self, chunk_pos: ChunkPosition) -> Option<ChunkType> {
        match self.cached_chunks.entry(chunk_pos) {
            Entry::Occupied(mut occ) => {
                let chunk_pos = occ.key().clone();
                let slot = occ.get_mut();
                if slot.is_none() {
                    let c = self.world_ref.dimension_mut(chunk_pos.dimension()).unwrap().chunk_mut(chunk_pos.position());
                    *slot = c;
                }
                slot.clone()
            }
            Entry::Vacant(_) => { None }
        }
    }
}

impl<'r, 'a> BlockReader<'a> for Selection<'r, 'a> {

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

            let mut relative_pos = (0, min_y, 0);
            let chunk_position = actual_chunk.position();
            let world_chunk_position = chunk_position_to_world_position(chunk_position.position(), chunk_size);

            for b in actual_chunk.block_store().blocks() {
                if !callback(FullBlock::new_with_data(
                    &self.world_ref.get(),
                    b,
                    Position::new(chunk_position.dimension(), world_chunk_position.0 + relative_pos.0, relative_pos.1, world_chunk_position.1 + relative_pos.2)
                )) { return; };

                relative_pos.0 = relative_pos.0 + 1;
                if relative_pos.0 % chunk_size == 0 {
                    relative_pos.0 = 0;
                    relative_pos.2 = relative_pos.2 + 1;
                    if relative_pos.2 % chunk_size == 0 {
                        relative_pos.2 = 0;
                        relative_pos.1 += 1;
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
                    return Some(FullBlock::new_with_data(&self.world_ref.get(), local_block, block_position));
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

impl<'r, 'a> EntityReader for Selection<'r, 'a> {
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
                        &self.world_ref.get(),
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

impl<'r, 'a> EntityWriter for Selection<'r, 'a> {
    fn set_entity_at_position(&mut self, entity: Entity) -> bool {
        let dim_id = entity.base().position().dimension();
        match self.world_ref.dimension_mut(dim_id) {
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

impl<'r, 'a> BlockWriter for Selection<'r, 'a> {
    fn set_block_at_position(&mut self, block: FullBlock) -> bool {
        if let Some((parent, rel_index)) = block.parent_chunk() {
            let mut ch = parent.lock().unwrap();
            ch.block_store_mut().set_block_at_index(rel_index, block.palette_block());
        }
        false
    }
}

pub struct SelectionBuilder<'a, 'r> {
    underlying: Selection<'r, 'a>,
}

impl<'a, 'r> SelectionBuilder<'a, 'r> {
    pub fn new(world: &'r mut World<'a>, version: &Arc<Version>) -> Self {
        SelectionBuilder {
            underlying: Selection::new(world, version.clone()),
        }
    }

    pub fn new_owned(world: &'r mut World<'a>, version: Arc<Version>) -> Self {
        SelectionBuilder {
            underlying: Selection::new(world, version),
        }
    }

    pub fn build(self) -> Selection<'r, 'a> {
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
            let dim = self.underlying.world_ref.dimension(dimension);
            if dim.is_none() { panic!("Invalid dimension"); }
            for chunk_pos in dim.unwrap().chunk_positions() {
                self.underlying.cached_chunks.insert(ChunkPosition::new(chunk_pos.0, chunk_pos.1, dimension.clone()), None);
            }
        }
        self
    }

    pub fn all_chunks(mut self) -> Self {
        {
            let dims: Vec<String> = self.underlying.world_ref.dimensions().keys().cloned().collect();

            for dim in dims {
                self = self.all_dimension_chunks(&dim);
            }
        }
        self
    }
}
