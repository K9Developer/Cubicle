use crate::constants::constants::{BIOME_CELL_SIZE, ZLIB_COMPRESSION_TYPE};
use crate::constants::versions::Version;
use crate::loaders::loader_utils::{get_region_files_in_folder, handle_chunk_compression, parse_region_file, uncompress_zlib};
use crate::models::nbt_structures::v3465::regular::{NBTBlockEntity, NBTBlockPalette, NBTChunk, NBTSection};
use crate::models::other::region::{Region, RegionType};
use crate::models::other::tick::Tick;
use crate::models::world::block::PaletteBlock;
use crate::models::world::chunk::Chunk;
use fastnbt::Value;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use crate::{transmute_map, I32};
use crate::loaders::templates::block_loader::BlockLoader;
use crate::loaders::v3465::utils::nbt_utils::{parse_nbt_item, parse_nbt_storage_container, parse_sign_text_block, parse_spawner_spawn_data, parse_spawner_spawn_potentials, take_bool, take_i16, take_i32, take_i32_vec, take_i8, take_inventory, take_list, take_long, take_map, take_string, take_text_component};
use crate::models::block_entity::block_entity::BlockEntity;
use crate::models::block_entity::prelude::*;
use crate::models::block_entity::types::cooker::cooker::CookerBlockEntity;
use crate::models::block_entity::types::lectern::LecternBlockEntity;
use crate::models::block_entity::types::storage_container::storage_container::StorageContainerBlockEntity;
use crate::models::other::lasso_string::LassoString;
use crate::models::other::properties::Properties;
use crate::models::positions::chunk_position::ChunkPosition;
use crate::models::positions::whole_position::Position;
use crate::models::stores::biome_store::BiomeStore;
use crate::models::stores::block_entity_store::BlockEntityStore;
use crate::models::stores::block_store::BlockStore;
use crate::models::stores::heightmap_store::HeightmapStore;
use crate::models::stores::structure_store::StructureStoreReference;
use crate::models::world::tile_tick::{TileTick, TileTickType};
use crate::models::world_structures::generic_structure::{BoundingBox, GenericChildStructure, GenericParentStructure};
use crate::types::{HeightmapKind, WorldKind};
use crate::utils::generic_utils::bit_length;
// TODO: Support other dimensions (custom paths)

pub struct BlockLoaderV3465 {
    pub version: Arc<Version>,
}

impl BlockLoaderV3465 {
    #[inline]
    unsafe fn parse_longs(&self, longs: Vec<u64>, bits_per_entry: u32, max_entries: usize, index_replacement_map: &[usize], output_slice: &mut [usize]) {
        let entries_per_long = (u64::BITS / bits_per_entry) as usize;
        let mut current_entry_count = 0;
        let mask: usize = (1 << bits_per_entry) - 1;

        for long_value in longs {
            let mut shifted_value = long_value as usize;

            for _ in 0..entries_per_long {
                if current_entry_count == max_entries { return; }
                let old_palette_index = shifted_value & mask;
                *output_slice.get_unchecked_mut(current_entry_count) = *index_replacement_map.get_unchecked(old_palette_index);
                shifted_value >>= bits_per_entry;
                current_entry_count += 1;
            }
        }
    }

    unsafe fn parse_section_blocks(&self, section: NBTSection, block_store: &mut BlockStore, section_block_count: usize) {
        let Some(block_states) = section.block_states else {return};
        let Some(block_palette) = block_states.palette else { return; };

        let mut index_replacement_map = Vec::with_capacity(block_palette.len());
        let old_palette_len = block_palette.len();

        let mut first_item_new_index = 0;

        for nbt_block in block_palette.into_iter() {
            let new_index = block_store.add_block_to_palette(PaletteBlock::new(&nbt_block.name, nbt_block.properties));
            if first_item_new_index == 0 { first_item_new_index = new_index; }
            index_replacement_map.push(new_index);
        }

        let start = ((section.y-self.version.data.lowest_y/self.version.data.section_height) * section_block_count as i32) as usize;
        let end = start + section_block_count;
        let indices = block_store.indices_slice_mut();
        let section_indices: &mut [usize] = &mut indices[start..end];

        if let Some(Value::LongArray(arr)) = block_states.data {
            let section_data_i64: Vec<i64> = arr.into_inner();
            let longs: Vec<u64> = section_data_i64.into_iter().map(|x| x as u64).collect();;
            let bits_per_block: u32 = max(bit_length(old_palette_len as i32 - 1), 4);

            self.parse_longs(longs, bits_per_block, section_block_count, index_replacement_map.as_slice(), section_indices);
        } else {
            section_indices.fill(first_item_new_index)
        }
    }

    unsafe fn parse_section_biomes(&self, mut section: NBTSection, biome_store: &mut BiomeStore, section_biome_count: usize) -> NBTSection {
        let sec_biomes = section.biomes.take();
        let Some(biome_states) = sec_biomes else { return section; };
        let Some(biome_palette) = biome_states.palette else { return section; };

        let mut index_replacement_map = Vec::with_capacity(biome_palette.len());
        let old_palette_len = biome_palette.len();

        let mut first_item_new_index = 0;

        for nbt_biome in biome_palette.into_iter() {
            let new_index = biome_store.add_biome_to_palette(nbt_biome);
            if first_item_new_index == 0 { first_item_new_index = new_index; }
            index_replacement_map.push(new_index);
        }

        let start = ((section.y-self.version.data.lowest_y/self.version.data.section_height) * section_biome_count as i32) as usize;
        let end = start + section_biome_count;
        let indices = biome_store.indices_slice_mut();
        let section_indices: &mut [usize] = &mut indices[start..end];

        if let Some(Value::LongArray(arr)) = biome_states.data {
            let section_data_i64: Vec<i64> = arr.into_inner();
            let longs: Vec<u64> = section_data_i64.into_iter().map(|x| x as u64).collect();
            let bits_per_biome: u32 = bit_length(old_palette_len as i32 - 1);

            self.parse_longs(longs, bits_per_biome, section_biome_count, index_replacement_map.as_slice(), section_indices);
        } else {
            section_indices.fill(first_item_new_index)
        }

        section
    }

    pub unsafe fn parse_chunk_heightmaps(&self, heightmap_nbt: Option<Value>, kind: HeightmapKind, store: &mut HeightmapStore) {
        if let Some(Value::LongArray(arr)) = heightmap_nbt {
            let heightmap_data: Vec<i64> = arr.into_inner();
            store.get_kind_mut(kind).set_via_longs(heightmap_data);
        }
    }

    pub unsafe fn parse_block_entities(&self, mut block_entity_nbt: NBTBlockEntity, dimension:  &LassoString, store: &mut BlockEntityStore) {
        let mut generic_be = GenericBlockEntity::new(
            block_entity_nbt.id,
            Position::new(dimension.clone(), block_entity_nbt.x, block_entity_nbt.y, block_entity_nbt.z),
            Properties::new(HashMap::new())
        );

        // TODO: This is a bit too hardcoded (extensions should be able to affect this).
        let props = &mut block_entity_nbt.others;
        let mut final_be: BlockEntity;
        match generic_be.id() {
            // Storage Container
            "minecraft:chest" | "minecraft:trapped_chest" | "minecraft:barrel" | "minecraft:shulker_box" => {
                final_be = BlockEntity::StorageContainer(StorageContainerBlockEntity::Normal(
                    parse_nbt_storage_container(generic_be, props, 27)
                ));
            },
            "minecraft:dropper" | "minecraft:dispenser" => {
                final_be = BlockEntity::StorageContainer(StorageContainerBlockEntity::Spitter(
                    parse_nbt_storage_container(generic_be, props, 9)
                ))
            },
            "minecraft:hopper" => {
                final_be = BlockEntity::StorageContainer(StorageContainerBlockEntity::Hopper(
                    HopperBlockEntity::new(
                        parse_nbt_storage_container(generic_be, props, 5),
                        take_i32(props, "TransferCooldown").unwrap_or(0)
                    )
                ))
            },
            "minecraft:chiseled_bookshelf" => {
                final_be = BlockEntity::StorageContainer(StorageContainerBlockEntity::ChiseledBookshelf(
                    ChiseledBookshelfBlockEntity::new(
                        parse_nbt_storage_container(generic_be, props, 5),
                        take_i32(props, "last_interacted_slot").unwrap_or(0)
                    )
                ))
            },

            // Cooker
            "minecraft:furnace" | "minecraft:smoker" | "minecraft:blast_furnace" => {
                let hist: Option<HashMap<String, i32>> = take_map(props, "RecipesUsed").and_then(|m| Some(transmute_map(m)));
                final_be = BlockEntity::Cooker(CookerBlockEntity::Furnace(
                    FurnaceBlockEntity::new(
                        generic_be,
                        take_inventory(props, "Items", 3),
                        hist.unwrap_or_default(),
                        take_string(props, "Lock"),
                        take_text_component(props, "CustomName"),
                        Tick::new(take_i16(props, "BurnTime").unwrap_or(0) as usize),
                        Tick::new(take_i16(props, "CookTime").unwrap_or(0) as usize),
                        Tick::new(take_i16(props, "CookTimeTotal").unwrap_or(0) as usize),
                    )
                ))
            },
            "minecraft:campfire" | "minecraft:soul_campfire" => {
                let cooking_times = take_i32_vec(props, "CookingTimes").unwrap_or_default().iter().map(|&i| Tick::new(i as usize)).collect::<Vec<Tick>>();
                let ticks_to_cook = take_i32_vec(props, "CookingTotalTimes").unwrap_or_default().iter().map(|&i| Tick::new(i as usize)).collect::<Vec<Tick>>();
                final_be = BlockEntity::Cooker(CookerBlockEntity::Campfire(
                    CampfireBlockEntity::new(
                        generic_be,
                        cooking_times,
                        ticks_to_cook,
                        take_inventory(props, "Items", 4),
                    )
                ))
            },
            "minecraft:brewing_stand" => {
                final_be = BlockEntity::Cooker(CookerBlockEntity::BrewingStand(
                    BrewingStandBlockEntity::new(
                        generic_be,
                        take_inventory(props, "Items", 5),
                        Tick::new(take_i16(props, "BrewTime").unwrap_or(0) as usize),
                        take_i8(props, "Fuel").unwrap_or(0),
                        take_string(props, "Lock"),
                        take_text_component(props, "CustomName"),
                    )
                ))
            },

            // Others
            "minecraft:lectern" => {
                final_be = BlockEntity::Lectern(
                    LecternBlockEntity::new(
                        generic_be,
                        parse_nbt_item(take_map(props, "Book")).and_then(|a| Some(a.1)),
                        take_i32(props, "Page").unwrap_or(0)
                    )
                )
            }

            "minecraft:spawner" => {
                final_be = BlockEntity::Spawner(
                    SpawnerBlockEntity::new(
                        generic_be,
                        Tick::new(take_i16(props, "Delay").unwrap_or(0) as usize),
                        take_i16(props, "MaxNearbyEntities").unwrap_or(0),
                        Tick::new(take_i16(props, "MaxSpawnDelay").unwrap_or(0) as usize),
                        Tick::new(take_i16(props, "MinSpawnDelay").unwrap_or(0) as usize),
                        take_i16(props, "RequiredPlayerRange").unwrap_or(0),
                        take_i16(props, "SpawnCount").unwrap_or(0),
                        take_i16(props, "SpawnRange").unwrap_or(0),
                        parse_spawner_spawn_data(take_map(props, "SpawnData"), dimension),
                        parse_spawner_spawn_potentials(take_list(props, "SpawnPotentials"), dimension).unwrap_or_default(),
                    )
                )
            },

            "minecraft:command_block" | "minecraft:repeating_command_block" | "minecraft:chain_command_block" => {
                if let (Some(auto), Some(cmd), Some(cond_met), last_ex, lo,
                        Some(pow), Some(succ), Some(to), Some(ule)) =
                    (take_bool(props, "auto"), take_string(props, "Command"), take_bool(props, "conditionMet"),
                     take_long(props, "LastExecution"), take_string(props, "LastOutput"), take_bool(props, "powered"),
                     take_i32(props, "SuccessCount"), take_bool(props, "TrackOutput"), take_bool(props, "UpdateLastExecution")) {
                    final_be = BlockEntity::CommandBlock(
                        CommandBlockBlockEntity::new(
                            generic_be,
                            auto,
                            cmd,
                            cond_met,
                            take_text_component(props, "CustomName"),
                            Tick::new(last_ex.unwrap_or_default() as usize),
                            lo.unwrap_or_default(),
                            pow,
                            succ,
                            to,
                            ule
                        )
                    )
                } else {
                    generic_be.set_properties(Properties::new(block_entity_nbt.others));
                    store.add_unchecked(BlockEntity::Other(generic_be));
                    return;
                }
            },

            "minecraft:sign" | "minecraft:hanging_sign" => {
                final_be = BlockEntity::Sign(
                    SignBlockEntity::new(
                        generic_be,
                        take_bool(props, "is_waxed").unwrap_or(false),
                        parse_sign_text_block(take_map(props, "front_text")),
                        parse_sign_text_block(take_map(props, "back_text"))
                    )
                )
            }

            &_ => {
                generic_be.set_properties(Properties::new(block_entity_nbt.others));
                store.add_unchecked(BlockEntity::Other(generic_be));
                return;
            }
        }
        final_be.base_mut().set_properties(Properties::new(block_entity_nbt.others));
        store.add_unchecked(final_be);
    }

    unsafe fn populate_chunk_with_blocks(&self, chunk_obj: &mut Chunk, mut chunk_nbt: NBTChunk, dimension: &LassoString) {
        // tile ticks
        for bt in chunk_nbt.block_ticks {
            let tile_tick = TileTick::new(Position::new(dimension.clone(), bt.x, bt.y, bt.z), bt.priority, bt.time_until_tick, TileTickType::BLOCK);
            chunk_obj.set_tile_tick(tile_tick);
        }
        for ft in chunk_nbt.fluid_ticks {
            let tile_tick = TileTick::new(Position::new(dimension.clone(), ft.x, ft.y, ft.z), ft.priority, ft.time_until_tick, TileTickType::FLUID);
            chunk_obj.set_tile_tick(tile_tick);
        }

        let (block_store, biome_store, heightmap_store, block_entity_store) = chunk_obj.stores_mut();
        let section_block_count = (self.version.data.section_height * self.version.data.chunk_size * self.version.data.chunk_size) as usize;
        let section_biome_count = section_block_count / BIOME_CELL_SIZE.pow(3) as usize;

        // block entities
        for mut be in chunk_nbt.block_entities {
            if let Some(be) = be.take() {
                self.parse_block_entities(be, dimension, block_entity_store);
            }
        }

        // heightmaps
        self.parse_chunk_heightmaps(chunk_nbt.heightmaps.ocean_floor.take(), HeightmapKind::Ground, heightmap_store);
        self.parse_chunk_heightmaps(chunk_nbt.heightmaps.motion_blocking.take(), HeightmapKind::MotionBlocking, heightmap_store);
        self.parse_chunk_heightmaps(chunk_nbt.heightmaps.motion_blocking_no_leaves.take(), HeightmapKind::MotionBlockingNoLeaves, heightmap_store);
        self.parse_chunk_heightmaps(chunk_nbt.heightmaps.world_surface.take(), HeightmapKind::SkyExposed, heightmap_store);

        // blocks and biomes
        for mut section in chunk_nbt.sections {
            section = self.parse_section_biomes(section, biome_store, section_biome_count);
            self.parse_section_blocks(section, block_store, section_block_count);
        }
    }

    fn populate_chunk_with_structures(&self, chunk_obj: &mut Chunk, mut chunk_nbt: &mut NBTChunk) -> Vec<GenericParentStructure> {
        let mut new_structures = Vec::<GenericParentStructure>::new();

        // refs
        for (structure_id, chunk_refs_val) in chunk_nbt.structures.references.iter()  {
            if let Value::LongArray(chunk_refs_val) = chunk_refs_val {
                let chunk_refs: &[i64] = &*chunk_refs_val;
                for chunk_ref in chunk_refs {
                    chunk_obj.add_structure(StructureStoreReference::new(chunk_ref.clone(), structure_id.clone()));
                }
            }
        }

        // actual
        for (_, structure) in chunk_nbt.structures.starts.take().unwrap() {
            let mut children = Vec::<GenericChildStructure>::new();

            if let Some(nbt_children) = structure.children {
                for child in nbt_children {
                    children.push(GenericChildStructure::new(
                        &*child.id,
                        BoundingBox::from_BB(child.bounding_box, chunk_obj.position().dimension().clone()),
                        Properties::new(child.others)
                    ));
                }
            }


            let parent = GenericParentStructure::new(
                chunk_obj.position().clone(),
                &*structure.id,
                children,
                Properties::new(structure.others)
            );

            new_structures.push(parent);
        }

        new_structures
    }
}

impl<'a> BlockLoader<'a> for BlockLoaderV3465 {
    fn get_region_files(&self, world_path: PathBuf) -> Vec<Region> {
        let overworld_region_folder = world_path.join((if self.version.world_type() == &WorldKind::Multiplayer { "world/" } else { "" }).to_owned() + "region");
        let nether_region_folder = world_path.join((if self.version.world_type() == &WorldKind::Multiplayer { "world/" } else { "" }).to_owned() + "DIM-1/region");
        let end_region_folder = world_path.join((if self.version.world_type() == &WorldKind::Multiplayer { "world/" } else { "" }).to_owned() + "DIM1/region");


        let mut regions = Vec::<Region>::new();

        regions.extend(get_region_files_in_folder(&overworld_region_folder, "overworld".into(), RegionType::Block));
        regions.extend(get_region_files_in_folder(&nether_region_folder, "the_nether".into(), RegionType::Block));
        regions.extend(get_region_files_in_folder(&end_region_folder, "the_end".into(), RegionType::Block));

        regions
    }

    fn parse_region(&self, region: &Region) -> (Vec<Chunk>, HashMap<i64, Vec<GenericParentStructure>>) {
        let parsed_chunks = parse_region_file(region);

        let mut chunks = Vec::with_capacity(parsed_chunks.len());
        let mut new_structures = HashMap::new();

        unsafe {
            for parsed_chunk in parsed_chunks {
                let chunk_data = self.parse_chunk(
                    parsed_chunk.raw_bytes,
                    parsed_chunk.compression_type,
                    region.position.dimension(),
                );
                if let Some(chunk_data) = chunk_data {
                    let chunk_ref = chunk_data.0.position().reference();
                    chunks.push(chunk_data.0);
                    new_structures.entry(chunk_ref).or_insert_with(Vec::new).extend(chunk_data.1);
                }
            }
        }
        (chunks, new_structures)
    }

    fn parse_chunk(
        &self,
        data: Vec<u8>,
        compression_type: u8,
        dimension: &LassoString,
    ) -> Option<(Chunk, Vec<GenericParentStructure>)> {

        let chunk_data = handle_chunk_compression(compression_type, data)?;
        let mut chunk_nbt: NBTChunk = fastnbt::from_bytes(chunk_data.as_slice()).expect("Failed to parse chunk data");

        let mut chunk = Chunk::with_store_capacity(
            ChunkPosition::new(
                chunk_nbt.x_pos,
                chunk_nbt.z_pos,
                dimension.clone(),
            ),
            chunk_nbt.data_version,
            Tick::new(chunk_nbt.inhabited_time as usize),
            Tick::new(chunk_nbt.last_update as usize),
            chunk_nbt.status.clone(),

            chunk_nbt.sections.len() * 20, // expected avg amount - can be optimized
            3, // expected avg amount - can be optimized
            chunk_nbt.block_entities.len(),

            &self.version,
        );

        let dim_id = chunk.position().dimension().clone();
        let structures = self.populate_chunk_with_structures(&mut chunk, &mut chunk_nbt);
        unsafe { self.populate_chunk_with_blocks(&mut chunk, chunk_nbt, &dim_id); }
        Some((chunk, structures))
    }
}
