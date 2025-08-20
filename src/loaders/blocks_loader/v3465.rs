use super::loader::BlockLoader;
use crate::constants::constants::ZLIB_COMPRESSION_TYPE;
use crate::constants::versions::Version;
use crate::loaders::utils::{get_region_files_in_folder, parse_region_file, uncompress_zlib};
use crate::models::nbt_structures::v3465::regular::{NBTBlockPalette, NBTChunk};
use crate::models::other::position::Position;
use crate::models::other::region::Region;
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
        Block::new(&*nbt_block.name, nbt_block.properties)
    }

    // TODO: There could be cases where the sections arent ordered correctly so we'll have to sort / do something faster with the y of the sections
    unsafe fn populate_chunk(&self, chunk_obj: &mut Chunk<'a>, chunk_nbt: NBTChunk) {
        let block_store = chunk_obj.block_store_mut();
        let mut block_index = 0;
        let section_block_count = (self.version.data.section_height
            * self.version.data.chunk_size
            * self.version.data.chunk_size) as usize;

        for section in chunk_nbt.sections {
            if section.block_states.palette.is_none() {
                block_index += section_block_count;
                continue;
            }
            let block_palette = section.block_states.palette.unwrap();
            let mut index_replacement_map = vec![0usize; block_palette.len()];

            for (old_index, nbt_block) in block_palette.iter().enumerate() {
                let new_index = block_store.add_nbt_block_to_palette_v3465(nbt_block);
                index_replacement_map[old_index] = new_index;
            }

            let mut section_palette_indices: Vec<usize> =
                vec![index_replacement_map[0] as usize; section_block_count];
            if let Some(Value::LongArray(arr)) = section.block_states.data.as_ref() {
                let section_data_i64: &[i64] = &*arr;
                let longs: &[u64] = unsafe {
                    std::slice::from_raw_parts(
                        section_data_i64.as_ptr() as *const u64,
                        section_data_i64.len(),
                    )
                };
                let bits_per_block: u32 = max(bit_length(block_palette.len() as i32 - 1), 4) as u32;
                let blocks_per_long = (u64::BITS / bits_per_block) as usize;
                let mask: u64 = (1 << bits_per_block) - 1;

                let mut section_block_index = 0;
                for &long_value in longs {
                    let mut shifted_value = long_value;
                    let blocks_to_process = min(
                        blocks_per_long,
                        section_block_count - section_block_index - 1,
                    );

                    for _ in 0..blocks_to_process {
                        let old_palette_index = shifted_value & mask;
                        unsafe {
                            *section_palette_indices.get_unchecked_mut(section_block_index) =
                                *index_replacement_map.get_unchecked(old_palette_index as usize);
                        }
                        shifted_value >>= bits_per_block;
                        section_block_index += 1;
                    }
                }
            }

            block_store.set_blocks_with_slice(block_index, section_palette_indices.as_slice());
            block_index += section_block_count;
        }
    }
}

impl<'a> BlockLoader<'a> for BlockLoaderV3465<'a> {
    fn get_region_files(&self, world_path: PathBuf) -> Vec<Region> {
        let overworld_region_folder = world_path.join(
            (if self.version.world_type() == &WorldType::MULTIPLAYER {
                "world/"
            } else {
                ""
            })
            .to_owned()
                + "region",
        );
        let nether_region_folder = world_path.join(
            (if self.version.world_type() == &WorldType::MULTIPLAYER {
                "world/"
            } else {
                ""
            })
            .to_owned()
                + "DIM-1/region",
        );
        let end_region_folder = world_path.join(
            (if self.version.world_type() == &WorldType::MULTIPLAYER {
                "world/"
            } else {
                ""
            })
            .to_owned()
                + "DIM1/region",
        );

        let mut regions = Vec::<Region>::new();

        regions.extend(get_region_files_in_folder(
            &overworld_region_folder,
            "overworld",
        ));
        regions.extend(get_region_files_in_folder(
            &nether_region_folder,
            "the_nether",
        ));
        regions.extend(get_region_files_in_folder(&end_region_folder, "the_end"));

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

        let chunk_nbt: NBTChunk =
            fastnbt::from_bytes(chunk_data.as_slice()).expect("Failed to parse chunk data");

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

        unsafe {
            self.populate_chunk(&mut chunk, chunk_nbt);
        }
        Some(chunk)
    }
}
