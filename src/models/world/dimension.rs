use std::collections::HashMap;
use crate::constants::versions::Version;
use crate::models::entity::entity::Entity;
use crate::models::other::position::Position;
use crate::models::other::structure::Structure;
use crate::models::world::chunk::Chunk;

pub struct Dimension<'a> {
    dimension_id: String,
    version: Version,

    chunks: HashMap<(i32,i32), Chunk<'a>>,
    entity_store: Vec<Entity>,
    structure_store: Vec<Structure>,
}

impl <'a> Dimension<'a> {
    fn new(dimension_id: String, version: Version) -> Dimension {
        Dimension {
            dimension_id,
            version,
            chunks: HashMap::new(),
            entity_store: Vec::new(),
            structure_store: Vec::new(),
        }
    }

    fn chunk(&mut self, chunk_position: Position) -> Option<&mut Chunk> {
        self.chunks.get_mut(&(chunk_position.i_x(), chunk_position.i_z()))
    }

    fn set_chunk(&mut self, chunk: Chunk) {
        let pos = chunk.position();
        self.chunks.insert((pos.i_x(), pos.i_z()), chunk);
    }

    fn add_entity(&mut self, entity: Entity) -> &Entity {
        self.entity_store.push(entity);
        self.entity_store.last().unwrap()
    }

    fn add_structure(&mut self, structure: Structure) -> &Structure {
        self.structure_store.push(structure);
        self.structure_store.last().unwrap()
    }
}