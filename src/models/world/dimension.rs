use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::constants::versions::Version;
use crate::models::stores::entity_store::{EntityStore, EntityStoreKey};
use crate::models::world::chunk::Chunk;
use crate::models::stores::structure_store::StructureStore;
use crate::models::world::selection::{Selection, SelectionBuilder};
use crate::models::world::world::World;
use crate::types::{ChunkType, WorldType};

pub struct Dimension {
    dimension_id: String,
    version: Arc<Version>,

    chunks: HashMap<(i32,i32), ChunkType>,
    entity_store: EntityStore,

    structure_store: StructureStore,
}

impl Dimension {
    pub fn new(dimension_id: String, version: Arc<Version>) -> Dimension {
        Dimension {
            entity_store: EntityStore::new(&dimension_id),
            dimension_id,
            version,
            chunks: HashMap::new(),
            structure_store: StructureStore::new(),
        }
    }

    pub fn structure_store(&self) -> &StructureStore { &self.structure_store }
    pub fn structure_store_mut(&mut self) -> &mut StructureStore { &mut self.structure_store }
    pub fn entity_store(&self) -> &EntityStore { &self.entity_store }
    pub fn entity_store_mut(&mut self) -> &mut EntityStore { &mut self.entity_store }

    pub fn chunk(&self, chunk_position: (i32, i32)) -> Option<ChunkType> { self.chunks.get(&chunk_position).cloned() }
    pub fn chunk_mut(&mut self, chunk_position: (i32, i32)) -> Option<ChunkType> { self.chunks.get_mut(&chunk_position).cloned() }
    pub fn chunks(&self) -> impl Iterator<Item = ChunkType> { self.chunks.values().cloned() }
    pub fn chunk_positions(&self) -> Vec<&(i32,i32)> { self.chunks.keys().collect() }

    pub fn chunk_count(&self) -> usize { self.chunks.len() }
    pub fn entity_count(&self) -> usize { self.entity_store.count() }

    pub fn set_chunk(&mut self, chunk: Chunk) -> ChunkType {
        let pos = chunk.position();
        let chunk = Arc::new(Mutex::new(chunk));
        let chunk_clone = chunk.clone();
        self.chunks.insert((pos.x(), pos.z()), chunk);
        chunk_clone
    }
    pub fn set_chunks(&mut self, chunks: Vec<Chunk>) {
        for chunk in chunks { self.set_chunk(chunk); }
    }
    pub fn delete_chunk(&mut self, chunk_position: (i32, i32)) -> Option<ChunkType> {
        self.chunks.remove(&chunk_position)
    }

    pub fn select<'r, 'a>(&self, world: &'r mut World<'a>) -> Selection<'r, 'a> {
        SelectionBuilder::new(world, &self.version).all_dimension_chunks(&self.dimension_id).build()
    }

}