use std::sync::Arc;
use crate::constants::versions::Version;
use crate::models::other::tick::Tick;
use crate::models::positions::chunk_position::ChunkPosition;
use crate::models::positions::whole_position::Position;
use crate::models::stores::biome_store::BiomeStore;
use crate::models::stores::block_entity_store::BlockEntityStore;
use crate::models::stores::block_store::BlockStore;
use crate::models::stores::entity_store::EntityStoreKey;
use crate::models::stores::heightmap_store::HeightmapStore;
use crate::models::stores::structure_store::StructureStoreReference;
use crate::models::world::tile_tick::TileTick;

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
    tile_ticks: Vec<TileTick>,
    block_entities: BlockEntityStore,
    entity_keys: Vec<EntityStoreKey>,

    structures: Vec<StructureStoreReference>,
}

impl Chunk {
    pub fn new(
        position: ChunkPosition,
        data_version: i32,
        inhabited_time: Tick,
        last_update: Tick,
        status: String,

        version: &Arc<Version>
    ) -> Self {
        Chunk::with_store_capacity(
            position, data_version, inhabited_time, last_update, status,
            20*16, 2, 20, // those numbers are random and can be optimized
            version,
        )
    }

    pub fn with_store_capacity(
        position: ChunkPosition,
        data_version: i32,
        inhabited_time: Tick,
        last_update: Tick,
        status: String,

        block_store_palette_capacity: usize,
        biome_store_palette_capacity: usize,
        block_entity_capacity: usize,

        version: &Arc<Version>,
    ) -> Self {
        Chunk {

            position,
            data_version,
            inhabited_time,
            last_update,
            status,

            block_store: BlockStore::with_palette_capacity(version.data.chunk_size, version.data.lowest_y, version.data.highest_y, block_store_palette_capacity),
            biome_store: BiomeStore::with_palette_capacity(version.data.chunk_size, version.data.lowest_y, version.data.highest_y, biome_store_palette_capacity),
            block_entities: BlockEntityStore::with_capacity(block_entity_capacity),
            heightmap_store: HeightmapStore::new(version.clone()),
            tile_ticks: Vec::new(),

            entity_keys: Vec::new(),
            structures: Vec::new(),
        }
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
    pub fn tile_ticks(&self) -> &Vec<TileTick> { &self.tile_ticks }

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

    pub fn set_tile_tick(&mut self, tile_tick: TileTick) {
        for (ind, tt) in self.tile_ticks.iter().enumerate() {
            if tt.position() == tile_tick.position() {
                self.tile_ticks[ind] = tile_tick;
                return;
            }
        }

        self.tile_ticks.push(tile_tick);
    }

    pub fn remove_tile_tick_at(&mut self, position: &Position) {
        for (ind, tt) in self.tile_ticks.iter().enumerate() {
            if tt.position() == position {
                self.tile_ticks.swap_remove(ind);
                return;
            }
        }
    }

    pub fn recalculate_heightmaps(&mut self) { todo!() } // TODO: Also needs to be called per column when a block changed or something
}
