use super::loader::BlockLoader;
use crate::constants::constants::ZLIB_COMPRESSION_TYPE;
use crate::constants::versions::Version;
use crate::loaders::utils::{get_region_files_in_folder, parse_region_file, uncompress_zlib};
use crate::models::nbt_structures::v3465::regular::{NBTBlockPalette, NBTChunk};
use crate::models::other::position::Position;
use crate::models::other::region::{Region, RegionType};
use crate::models::other::tick::Tick;
use crate::models::world::block::Block;
use crate::models::world::chunk::Chunk;
use crate::models::world::world::WorldType;
use crate::utils::bit_length;
use fastnbt::Value;
use std::cmp::{max, min};
use std::path::PathBuf;
// TODO: Support other dimensions (custom paths)

pub(super) struct BlockLoaderV3465<'a> {
    pub(crate) version: &'a Version,
}

impl<'a> BlockLoaderV3465<'a> {
    fn palette_block_to_block(&self, nbt_block: NBTBlockPalette) -> Block {
        Block::new(nbt_block.name.as_ref(), nbt_block.properties)
    }

    // TODO: Optimize the block palette loading
    unsafe fn populate_chunk(&self, chunk_obj: &mut Chunk<'a>, chunk_nbt: NBTChunk) {
        let block_store = chunk_obj.block_store_mut();
        let section_block_count = (self.version.data.section_height * self.version.data.chunk_size * self.version.data.chunk_size) as usize;

        for section in chunk_nbt.sections {

            let Some(block_palette) = section.block_states.palette else { continue; };

            let mut index_replacement_map = Vec::with_capacity(block_palette.len());
            let old_palette_len = block_palette.len();

            for nbt_block in block_palette.into_iter() {
                let new_index = block_store.add_block_to_palette(Block::new(&nbt_block.name, nbt_block.properties));
                index_replacement_map.push(new_index);
            }

            if let Some(Value::LongArray(arr)) = section.block_states.data.as_ref() {
                let section_data_i64: &[i64] = &*arr;
                let longs: &[u64] = unsafe {
                    std::slice::from_raw_parts(
                        section_data_i64.as_ptr() as *const u64,
                        section_data_i64.len(),
                    )
                };
                let bits_per_block: u32 = max(bit_length(old_palette_len as i32 - 1), 4);
                let blocks_per_long = (u64::BITS / bits_per_block) as usize;
                let mask: usize = (1 << bits_per_block) - 1;

                let mut relative_section_block_index = 0;
                let start = ((section.y-self.version.data.lowest_y/16) * section_block_count as i32) as usize;
                let end = start + section_block_count;

                let indices = block_store.indices_slice_mut();
                let section_indices: &mut [usize] = &mut indices[start..end];

                for &long_value in longs {
                    let mut shifted_value = long_value as usize;
                    let blocks_to_process = min(blocks_per_long, section_block_count - relative_section_block_index);

                    for _ in 0..blocks_to_process {
                        let old_palette_index = shifted_value & mask;
                        *section_indices.get_unchecked_mut(relative_section_block_index) = *index_replacement_map.get_unchecked(old_palette_index);
                        shifted_value >>= bits_per_block;
                        relative_section_block_index += 1;
                    }
                }
            }
        }
    }
}

impl<'a> BlockLoader<'a> for BlockLoaderV3465<'a> {
    fn get_region_files(&self, world_path: PathBuf) -> Vec<Region> {
        let overworld_region_folder = world_path.join((if self.version.world_type() == &WorldType::MULTIPLAYER { "world/" } else { "" }).to_owned() + "region");
        let nether_region_folder = world_path.join((if self.version.world_type() == &WorldType::MULTIPLAYER { "world/" } else { "" }).to_owned() + "DIM-1/region");
        let end_region_folder = world_path.join((if self.version.world_type() == &WorldType::MULTIPLAYER { "world/" } else { "" }).to_owned() + "DIM1/region");


        let mut regions = Vec::<Region>::new();

        regions.extend(get_region_files_in_folder(&overworld_region_folder, "overworld", RegionType::Block));
        regions.extend(get_region_files_in_folder(&nether_region_folder, "the_nether", RegionType::Block));
        regions.extend(get_region_files_in_folder(&end_region_folder, "the_end", RegionType::Block));

        regions
    }

    fn parse_region(&self, region: &Region) -> Vec<Chunk<'a>> {
        let parsed_chunks = parse_region_file(region);
        let mut chunks = Vec::with_capacity(parsed_chunks.len());
        for parsed_chunk in parsed_chunks {
            let chunk = self.parse_chunk(
                parsed_chunk.raw_bytes,
                parsed_chunk.compression_type,
                region.position.dimension(),
            );
            if chunk.is_some() {
                chunks.push(chunk.unwrap());
            }
        }
        chunks
    }

    fn parse_chunk(
        &self,
        data: Vec<u8>,
        compression_type: u8,
        dimension: &str,
    ) -> Option<Chunk<'a>> {
        let mut chunk_data = data;

        match compression_type {
            ZLIB_COMPRESSION_TYPE => {
                chunk_data = uncompress_zlib(chunk_data);
            }
            _ => return None,
        }

        let chunk_nbt: NBTChunk = fastnbt::from_bytes(chunk_data.as_slice()).expect("Failed to parse chunk data");

        let mut chunk = Chunk::new(
            self.version,
            Position::new(
                dimension,
                chunk_nbt.x_pos as f32,
                0f32,
                chunk_nbt.z_pos as f32,
            ),
            chunk_nbt.data_version,
            Tick::new(chunk_nbt.inhabited_time as usize),
            Tick::new(chunk_nbt.last_update as usize),
            chunk_nbt.status.clone(),
        );
        unsafe { self.populate_chunk(&mut chunk, chunk_nbt); }
        Some(chunk)
    }
}
