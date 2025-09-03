use super::loader::BlockLoader;
use crate::constants::constants::{BIOME_CELL_SIZE, ZLIB_COMPRESSION_TYPE};
use crate::constants::versions::Version;
use crate::loaders::utils::{get_region_files_in_folder, parse_region_file, uncompress_zlib};
use crate::models::nbt_structures::v3465::regular::{NBTBlockPalette, NBTChunk, NBTSection};
use crate::models::other::region::{Region, RegionType};
use crate::models::other::tick::Tick;
use crate::models::world::block::Block;
use crate::models::world::chunk::Chunk;
use crate::models::world::world::WorldKind;
use fastnbt::Value;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use crate::models::other::properties::Properties;
use crate::models::positions::chunk_position::ChunkPosition;
use crate::models::stores::biome_store::BiomeStore;
use crate::models::stores::block_store::BlockStore;
use crate::models::stores::structure_store::StructureStoreReference;
use crate::models::world_structures::generic_structure::{BoundingBox, GenericChildStructure, GenericParentStructure};
use crate::utils::generic_utils::bit_length;
// TODO: Support other dimensions (custom paths)

pub(super) struct BlockLoaderV3465 {
    pub(crate) version: Arc<Version>,
}

impl BlockLoaderV3465 {
    fn palette_block_to_block(&self, nbt_block: NBTBlockPalette) -> Block {
        Block::new(nbt_block.name.as_ref(), nbt_block.properties)
    }

    unsafe fn parse_longs(&self, longs: &[u64], bits_per_entry: u32, max_entries: usize, index_replacement_map: &[usize], output_slice: &mut [usize]) {
        let entries_per_long = (u64::BITS / bits_per_entry) as usize;
        let mut current_entry_count = 0;
        let mask: usize = (1 << bits_per_entry) - 1;

        for &long_value in longs {
            let mut shifted_value = long_value as usize;
            let blocks_to_process = min(entries_per_long, max_entries - current_entry_count);


            for _ in 0..blocks_to_process {
                let old_palette_index = shifted_value & mask;
                *output_slice.get_unchecked_mut(current_entry_count) = *index_replacement_map.get_unchecked(old_palette_index);
                shifted_value >>= bits_per_entry;
                current_entry_count += 1;
            }
        }
    }

    // TODO: Optimize the block palette loading
    unsafe fn parse_section_blocks(&self, section: NBTSection, block_store: &mut BlockStore, section_block_count: usize) {
        let Some(block_palette) = section.block_states.palette else { return; };

        let mut index_replacement_map = Vec::with_capacity(block_palette.len());
        let old_palette_len = block_palette.len();

        let mut first_item_new_index = 0;

        for nbt_block in block_palette.into_iter() {
            let new_index = block_store.add_block_to_palette(Block::new(&nbt_block.name, nbt_block.properties));
            if first_item_new_index == 0 { first_item_new_index = new_index; }
            index_replacement_map.push(new_index);
        }

        let start = ((section.y-self.version.data.lowest_y/self.version.data.section_height) * section_block_count as i32) as usize;
        let end = start + section_block_count;
        let indices = block_store.indices_slice_mut();
        let section_indices: &mut [usize] = &mut indices[start..end];

        if let Some(Value::LongArray(arr)) = section.block_states.data.as_ref() {
            let section_data_i64: &[i64] = &*arr;
            let longs: &[u64] = unsafe { std::slice::from_raw_parts(section_data_i64.as_ptr() as *const u64, section_data_i64.len(), ) };
            let bits_per_block: u32 = max(bit_length(old_palette_len as i32 - 1), 4);

            self.parse_longs(longs, bits_per_block, section_block_count, index_replacement_map.as_slice(), section_indices);
        } else {
            section_indices.fill(first_item_new_index)
        }
    }

    unsafe fn parse_section_biomes(&self, section: &NBTSection, biome_store: &mut BiomeStore, section_biome_count: usize) {
        let Some(biome_palette) = section.biomes.palette.clone() else { return; };

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

        if let Some(Value::LongArray(arr)) = section.biomes.data.as_ref() {
            let section_data_i64: &[i64] = &*arr;
            let longs: &[u64] = unsafe { std::slice::from_raw_parts(section_data_i64.as_ptr() as *const u64, section_data_i64.len(), ) };
            let bits_per_biome: u32 = bit_length(old_palette_len as i32 - 1);

            self.parse_longs(longs, bits_per_biome, section_biome_count, index_replacement_map.as_slice(), section_indices);
        } else {
            section_indices.fill(first_item_new_index)
        }
    }

    unsafe fn populate_chunk_with_blocks(&self, chunk_obj: &mut Chunk, chunk_nbt: NBTChunk) {
        let (block_store, biome_store) = chunk_obj.stores_mut();

        let section_block_count = (self.version.data.section_height * self.version.data.chunk_size * self.version.data.chunk_size) as usize;
        let section_biome_count = section_block_count / BIOME_CELL_SIZE.pow(3) as usize;

        for section in chunk_nbt.sections {
            self.parse_section_biomes(&section, biome_store, section_biome_count);
            self.parse_section_blocks(section, block_store, section_block_count);
        }
    }

    fn populate_chunk_with_structures(&self, chunk_obj: &mut Chunk, chunk_nbt: &NBTChunk) -> Vec<GenericParentStructure> {
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
        for (_, structure) in chunk_nbt.structures.starts.iter() {
            let mut children = Vec::<GenericChildStructure>::new();

            if let Some(nbt_children) = &structure.children {
                for child in nbt_children {
                    children.push(GenericChildStructure::new(
                        &*child.id,
                        BoundingBox::from_BB(child.bounding_box.clone(), chunk_obj.position().dimension()),
                        Properties::new(child.others.clone()) // TODO: I really dont like this - its a structure too so very slow. The thing is chunk_nbt is owned by the other func
                    ));
                }
            }


            let parent = GenericParentStructure::new(
                chunk_obj.position().clone(),
                &*structure.id,
                children,
                Properties::new(structure.others.clone())
            );

            new_structures.push(parent);
        }

        new_structures
    }
}

impl<'a> BlockLoader<'a> for BlockLoaderV3465 {
    fn get_region_files(&self, world_path: PathBuf) -> Vec<Region> {
        let overworld_region_folder = world_path.join((if self.version.world_type() == &WorldKind::MULTIPLAYER { "world/" } else { "" }).to_owned() + "region");
        let nether_region_folder = world_path.join((if self.version.world_type() == &WorldKind::MULTIPLAYER { "world/" } else { "" }).to_owned() + "DIM-1/region");
        let end_region_folder = world_path.join((if self.version.world_type() == &WorldKind::MULTIPLAYER { "world/" } else { "" }).to_owned() + "DIM1/region");


        let mut regions = Vec::<Region>::new();

        regions.extend(get_region_files_in_folder(&overworld_region_folder, "overworld", RegionType::Block));
        regions.extend(get_region_files_in_folder(&nether_region_folder, "the_nether", RegionType::Block));
        regions.extend(get_region_files_in_folder(&end_region_folder, "the_end", RegionType::Block));

        regions
    }

    fn parse_region(&self, region: &Region) -> (Vec<Chunk<'a>>, HashMap<i64, Vec<GenericParentStructure>>) {
        let parsed_chunks = parse_region_file(region);

        let mut chunks = Vec::with_capacity(parsed_chunks.len());
        let mut new_structures = HashMap::new();

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
        (chunks, new_structures)
    }

    fn parse_chunk(
        &self,
        data: Vec<u8>,
        compression_type: u8,
        dimension: &str,
    ) -> Option<(Chunk<'a>, Vec<GenericParentStructure>)> {
        let mut chunk_data = data;

        match compression_type {
            ZLIB_COMPRESSION_TYPE => {
                chunk_data = uncompress_zlib(chunk_data);
            }
            _ => return None,
        }

        let chunk_nbt: NBTChunk = fastnbt::from_bytes(chunk_data.as_slice()).expect("Failed to parse chunk data");

        let mut chunk = Chunk::new(
            self.version.clone(),
            ChunkPosition::new(
                chunk_nbt.x_pos,
                chunk_nbt.z_pos,
                dimension,
            ),
            chunk_nbt.data_version,
            Tick::new(chunk_nbt.inhabited_time as usize),
            Tick::new(chunk_nbt.last_update as usize),
            chunk_nbt.status.clone(),
        );

        let structures = self.populate_chunk_with_structures(&mut chunk, &chunk_nbt);
        unsafe { self.populate_chunk_with_blocks(&mut chunk, chunk_nbt); }
        Some((chunk, structures))
    }
}
