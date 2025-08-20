use std::collections::HashMap;
use crate::constants::versions::Version;
use crate::models::entity::entity::Entity;
use crate::models::other::position::Position;
use crate::models::other::structure::Structure;
use crate::models::world::chunk::Chunk;

pub struct Dimension<'a> {
    dimension_id: String,
    version: &'a Version,

    chunks: HashMap<(i32,i32), Chunk<'a>>,
    entity_store: Vec<Entity>,
    structure_store: Vec<Structure>,
}

impl <'a> Dimension<'a> {
    pub fn new(dimension_id: String, version: &'a Version) -> Dimension<'a> {
        Dimension {
            dimension_id,
            version,
            chunks: HashMap::new(),
            entity_store: Vec::new(),
            structure_store: Vec::new(),
        }
    }

    pub fn chunk(&self, chunk_position: (i32, i32)) -> Option<&Chunk<'a>> {
        self.chunks.get(&chunk_position)
    }

    pub fn chunk_mut(&mut self, chunk_position: (i32, i32)) -> Option<&mut Chunk<'a>> {
        self.chunks.get_mut(&chunk_position)
    }

    pub fn set_chunk(&mut self, chunk: Chunk<'a>) {
        let pos = chunk.position();
        self.chunks.insert((pos.i_x(), pos.i_z()), chunk);
    }

    pub fn set_chunks(&mut self, chunks: Vec<Chunk<'a>>) {
        for chunk in chunks { self.set_chunk(chunk); }
    }

    pub fn add_entity(&mut self, entity: Entity) -> &Entity {
        self.entity_store.push(entity);
        self.entity_store.last().unwrap()
    }

    pub fn add_structure(&mut self, structure: Structure) -> &Structure {
        self.structure_store.push(structure);
        self.structure_store.last().unwrap()
    }
}