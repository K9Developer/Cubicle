use std::sync::Arc;
use crate::constants::versions::Version;
use crate::models::other::tick::Tick;
use crate::models::positions::chunk_position::ChunkPosition;
use crate::models::stores::biome_store::BiomeStore;
use crate::models::stores::block_store::BlockStore;
use crate::models::stores::entity_store::EntityStoreKey;
use crate::models::stores::heightmap_store::HeightmapStore;
use crate::models::stores::structure_store::StructureStoreReference;
// TODO: have multiple selector like EntitySelector, then have stuff like remove_entity(selector: EntitySelector) - this will be used in filters too.

#[derive(Debug)]
pub struct Chunk {
    position: ChunkPosition,
    data_version: i32,
    inhabited_time: Tick,
    last_update: Tick,
    status: String,

    block_store: BlockStore,
    biome_store: BiomeStore,
    heightmap_store: HeightmapStore,

    entity_keys: Vec<EntityStoreKey>,
    structures: Vec<StructureStoreReference>,

    version: Arc<Version>,
}

impl Chunk {
    pub fn new(
        version: Arc<Version>,
        position: ChunkPosition,
        data_version: i32,
        inhabited_time: Tick,
        last_update: Tick,
        status: String,
    ) -> Self {
        Chunk {

            position,
            data_version,
            inhabited_time,
            last_update,
            status,

            block_store: BlockStore::new(version.clone()),
            biome_store: BiomeStore::new(version.clone()),
            heightmap_store: HeightmapStore::new(version.clone()),
            version,

            entity_keys: Vec::new(),
            structures: Vec::new(),
        }
    }

    pub fn version(&self) -> &Version {
        &self.version
    }
    pub fn position(&self) -> &ChunkPosition {
        &self.position
    }
    pub fn inhabited_time(&self) -> &Tick {
        &self.inhabited_time
    }
    pub fn last_update(&self) -> &Tick {
        &self.last_update
    }
    pub fn status(&self) -> &String {
        &self.status
    }

    pub fn stores(&self) -> (&BlockStore, &BiomeStore, &HeightmapStore) { (&self.block_store, &self.biome_store, &self.heightmap_store) }
    pub fn stores_mut(&mut self) -> (&mut BlockStore, &mut BiomeStore, &mut HeightmapStore) { (&mut self.block_store, &mut self.biome_store, &mut self.heightmap_store) }
    pub fn block_store(&self) -> &BlockStore {
        &self.block_store
    }
    pub fn block_store_mut(&mut self) -> &mut BlockStore { &mut self.block_store }
    pub fn biome_store(&self) -> &BiomeStore { &self.biome_store }
    pub fn biome_store_mut(&mut self) -> &mut BiomeStore { &mut self.biome_store }
    pub fn heightmap_store(&self) -> &HeightmapStore { &self.heightmap_store }
    pub fn heightmap_store_mut(&mut self) -> &mut HeightmapStore { &mut self.heightmap_store }

    pub fn entity_keys(&self) -> &Vec<EntityStoreKey> { &self.entity_keys }
    pub fn entity_count(&self) -> usize { self.entity_keys.len() }
    pub fn structures(&mut self) -> &Vec<StructureStoreReference> {
        &self.structures
    }

    pub fn set_inhabited_time(&mut self, inhabited_time: Tick) {
        self.inhabited_time = inhabited_time;
    }
    pub fn set_last_update(&mut self, last_update: Tick) {
        self.last_update = last_update;
    }
    pub fn set_status(&mut self, status: &str) {
        self.status = status.to_string();
    }
    pub fn set_block_store(&mut self, block_store: BlockStore) {
        self.block_store = block_store;
    }
    pub fn set_entities(&mut self, entity_keys: Vec<EntityStoreKey>) { self.entity_keys = entity_keys; }

    pub fn add_entity(&mut self, entity_key: EntityStoreKey) { self.entity_keys.push(entity_key); }
    pub fn add_structure(&mut self, structure: StructureStoreReference) { self.structures.push(structure); }

    pub fn recalculate_heightmaps(&mut self) { todo!() } // TODO: Also needs to be called per column when a block changed or something
}
