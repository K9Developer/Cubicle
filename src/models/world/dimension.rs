use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::constants::versions::Version;
use crate::models::entity::entity::Entity;
use crate::models::world_structures::generic_structure::GenericParentStructure;
use crate::models::world::chunk::Chunk;
use crate::models::stores::structure_store::StructureStore;
use crate::types::ChunkType;

pub struct Dimension<'a> {
    dimension_id: String,
    version: Arc<Version>,

    chunks: HashMap<(i32,i32), ChunkType<'a>>,
    entity_store: Vec<Entity>,

    structure_store: StructureStore,
}

impl <'a> Dimension<'a> {
    pub fn new(dimension_id: String, version: Arc<Version>) -> Dimension<'a> {
        Dimension {
            dimension_id,
            version,
            chunks: HashMap::new(),
            entity_store: Vec::new(),
            structure_store: StructureStore::new(),
        }
    }

    pub fn structure_store(&self) -> &StructureStore { &self.structure_store }
    pub fn structure_store_mut(&mut self) -> &mut StructureStore { &mut self.structure_store }
    pub fn chunk(&self, chunk_position: (i32, i32)) -> Option<ChunkType<'a>> { self.chunks.get(&chunk_position).cloned() }

    pub fn entities(&self) -> Vec<&Entity> {
        self.entity_store.iter().collect()
    }

    pub fn chunk_mut(&mut self, chunk_position: (i32, i32)) -> Option<ChunkType<'a>> {
        self.chunks.get_mut(&chunk_position).cloned()
    }

    pub fn set_chunk(&mut self, chunk: Chunk<'a>) {
        let pos = chunk.position();
        self.chunks.insert((pos.x(), pos.z()), Arc::new(Mutex::new(chunk)));
    }

    pub fn set_chunks(&mut self, chunks: Vec<Chunk<'a>>) {
        for chunk in chunks { self.set_chunk(chunk); }
    }

    pub fn add_entity(&mut self, entity: Entity) -> &Entity {
        self.entity_store.push(entity);
        self.entity_store.last().unwrap()
    }

    pub fn add_entities(&mut self, entities: Vec<Entity>) {
        self.entity_store.extend(entities);
    }

}