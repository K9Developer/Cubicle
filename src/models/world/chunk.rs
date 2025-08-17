use crate::constants::versions::Version;
use crate::models::other::position::Position;
use crate::models::world::block_store::BlockStore;
use crate::models::entity::entity::Entity;
use crate::models::other::structure::Structure;
use crate::models::other::tick::Tick;

// TODO: have multiple selector like EntitySelector, then have stuff like remove_entity(selector: EntitySelector) - this will be used in filters too.

enum ChunkStatus {
    Empty,
    StructureStarts,
    StructureReferences,
    Biomes,
    Noise,
    Surface,
    Carvers,
    Features,
    Light,
    Spawn,
    Full
}

pub struct Chunk<'a> {
    position: Position,
    data_version: i32,
    inhabited_time: Tick,
    last_update: Tick,
    status: ChunkStatus,

    block_store: BlockStore<'a>,
    entities: Vec<&'static Entity>, // TODO: check if this lifetime is alright
    structures: Vec<&'static Structure>,

    version: &'a Version
}

impl<'a> Chunk<'a> {
    pub fn new(version: &'a Version, position: Position, data_version: i32, inhabited_time: Tick, last_update: Tick, status: ChunkStatus) -> Chunk {
        Chunk {
            version,

            position,
            data_version,
            inhabited_time,
            last_update,
            status,

            block_store: BlockStore::new(&version),
            entities: Vec::new(),
            structures: Vec::new()
      }
    }

    pub fn version(&self) -> &Version { &self.version }
    pub fn position(&self) -> &Position { &self.position }
    pub fn inhabited_time(&self) -> &Tick { &self.inhabited_time }
    pub fn last_update(&self) -> &Tick { &self.last_update }
    pub fn status(&self) -> &ChunkStatus { &self.status }
    pub fn block_store(&mut self) -> &mut BlockStore { &mut self.block_store }
    pub fn entities(&mut self) -> &mut Vec<&'static Entity> { &mut self.entities }
    pub fn structures(&mut self) -> &mut Vec<&'static Structure> { &mut self.structures }

    pub fn set_inhabited_time(&mut self, inhabited_time: Tick) { self.inhabited_time = inhabited_time; }
    pub fn set_last_update(&mut self, last_update: Tick) { self.last_update = last_update; }
    pub fn set_status(&mut self, status: ChunkStatus) { self.status = status; }
    pub fn set_block_store(&mut self, block_store: BlockStore<'a>) { self.block_store = block_store; }
    pub fn set_entities(&mut self, entities: Vec<&'static Entity>) { self.entities = entities; }

    pub fn add_entity(&mut self, entity: &'static Entity) { self.entities.push(entity); }
    pub fn add_structure(&mut self, structure: &'static Structure) { self.structures.push(structure); }
}