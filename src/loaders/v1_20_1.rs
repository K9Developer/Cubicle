use std::cmp::{max, min, PartialEq};
use std::path::PathBuf;
use crate::loaders::loader::Loader;
use crate::models::other::position::Position;
use crate::models::other::region::Region;
use crate::models::world::chunk::{Chunk};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::time::Instant;
use fastnbt::Value;
use crate::models::nbt_structures::v1_20_1::{NBTBlockPalette, NBTChunk};
use flate2::read::ZlibDecoder;
use crate::constants::constants::{MCA_REGION_LOCATION_SECTOR_ENTRY_SIZE, MCA_REGION_SECTOR_SIZE};
use crate::constants::versions::Version;
use crate::models::other::tick::Tick;
use crate::models::world::block::Block;
use crate::models::world::world::WorldType;
use crate::utils::bit_length;
// TODO: Support other dimensions (custom paths)

pub(super) struct WorldLoaderV012001<'a> {
    pub(crate) version: &'a Version
}

impl<'a> WorldLoaderV012001<'a> {
    fn get_region_files_in_folder(&self, path: PathBuf, dimension_name: &str) -> Vec<Region> {
        let mut regions = Vec::<Region>::new();
        if !path.exists() {
            println!("{} does not exist, skipping...", path.display());
            return Vec::new();
        }

        for file in path.read_dir().expect("Failed to read folder") {
            let file = file.unwrap();
            let file_name = file.file_name().into_string().unwrap();
            let file_parts: Vec<&str> = file_name.split('.').collect::<Vec<&str>>();

            if file_parts.len() != 4 {
                println!("{} file doesnt match the region file naming conversion. Skipping...", file.path().display());
                continue;
            }
            let region_x = file_parts[1].parse::<i32>().ok();
            let region_z = file_parts[2].parse::<i32>().ok();
            if region_x.is_none() || region_z.is_none() {
                println!("{} file doesnt match the region file naming conversion. Skipping...", file.path().display());
                continue;
            }
            regions.push(Region {
                position: Position::new(dimension_name, region_x.unwrap() as f32, 0f32, region_z.unwrap() as f32),
                path: file.path().to_path_buf()
            })
        }

        regions
    }

    fn uncompress_zlib(&self, data: Vec<u8>) -> Vec<u8> {
        let mut decoder = ZlibDecoder::new(&data[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).unwrap();
        decompressed
    }

    fn palette_block_to_block(&self, nbt_block: NBTBlockPalette) -> Block {
        Block::new(&*nbt_block.name, nbt_block.properties)
    }

    // TODO: Optimize the shit out of this - didnt want to overthink and just wanted this to work
    // TODO: There could be cases where the sections arent ordered correctly so we'll have to sort / do something faster with the y of the sections
    unsafe fn populate_chunk(&self, chunk_obj: &mut Chunk<'a>, chunk_nbt: NBTChunk) {

        let mut block_store = chunk_obj.block_store_mut();
        let mut block_index = 0;
        let section_block_count = (self.version.data.section_height * self.version.data.chunk_size * self.version.data.chunk_size) as usize;

        for section in chunk_nbt.sections {
            if section.block_states.palette.is_none() {
                block_index += section_block_count;
                continue;
            }
            let block_palette = section.block_states.palette.unwrap();
            let mut index_replacement_map = vec![0usize; block_palette.len()];

            for (old_index, nbt_block) in block_palette.iter().enumerate() {
                let new_index = block_store.add_nbt_block_to_palette(nbt_block);
                index_replacement_map[old_index] = new_index;
            }

            let mut section_palette_indices: Vec<usize> = vec![index_replacement_map[0] as usize; section_block_count];
            if let Some(Value::LongArray(arr)) = section.block_states.data.as_ref() {
                let section_data_i64: &[i64] = &*arr;
                let longs: &[u64] = unsafe { std::slice::from_raw_parts(section_data_i64.as_ptr() as *const u64, section_data_i64.len()) };
                let bits_per_block: u32 = max(bit_length(block_palette.len() as i32 - 1), 4) as u32;
                let blocks_per_long = (u64::BITS / bits_per_block) as usize;
                let mask: u64 = (1 << bits_per_block) - 1;

                let mut section_block_index = 0;
                for &long_value in longs {
                    let mut shifted_value = long_value;
                    let blocks_to_process = min(blocks_per_long, section_block_count - section_block_index - 1);

                    for _ in 0..blocks_to_process {
                        let old_palette_index = shifted_value & mask;
                        *section_palette_indices.get_unchecked_mut(section_block_index) = *index_replacement_map.get_unchecked(old_palette_index as usize);
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


impl<'a> Loader<'a> for WorldLoaderV012001<'a> {
    fn get_region_files(&self, world_path: PathBuf) -> Vec<Region> {
        let overworld_region_folder = world_path.join((if self.version.world_type() == &WorldType::MULTIPLAYER {"world/"} else {""}).to_owned() + "region");
        let nether_region_folder = world_path.join((if self.version.world_type() == &WorldType::MULTIPLAYER {"world/"} else {""}).to_owned() + "DIM-1/region");
        let end_region_folder = world_path.join((if self.version.world_type() == &WorldType::MULTIPLAYER {"world/"} else {""}).to_owned() + "DIM1/region");

        let mut regions = Vec::<Region>::new();

        regions.extend(self.get_region_files_in_folder(overworld_region_folder, "overworld"));
        regions.extend(self.get_region_files_in_folder(nether_region_folder, "the_nether"));
        regions.extend(self.get_region_files_in_folder(end_region_folder, "the_end"));

        regions
    }

    fn parse_region(&self, region: &Region) -> Vec<Chunk<'a>> {


        let mut file = File::open(region.path.clone()).expect("Failed to open region file");
        println!("{:?}", region.path);

        let mut loc_table = [0u8; MCA_REGION_SECTOR_SIZE];
        file.read_exact(&mut loc_table).expect("Failed to read locations table");
        let mut offsets = Vec::<usize>::with_capacity(MCA_REGION_SECTOR_SIZE/MCA_REGION_LOCATION_SECTOR_ENTRY_SIZE);

        for i in 0..(MCA_REGION_SECTOR_SIZE/MCA_REGION_LOCATION_SECTOR_ENTRY_SIZE) {
            let b0 = loc_table[i * 4];
            let b1 = loc_table[i * 4 + 1];
            let b2 = loc_table[i * 4 + 2];
            let b3 = loc_table[i * 4 + 3]; // sector count

            let entry = u32::from_be_bytes([b0, b1, b2, b3]);
            let sector_offset = (entry >> 8) as usize; // top 24 bits
            let sector_count  = (entry & 0xFF) as u8;
            if sector_offset != 0 && sector_count != 0 { offsets.push(sector_offset * MCA_REGION_SECTOR_SIZE); }
        }

        let mut chunks = Vec::with_capacity(offsets.len());
        println!("Loading {} chunks...", offsets.len());

        for offset in offsets {
            file.seek(SeekFrom::Start(offset as u64)).expect("Failed to seek");

            let mut raw_chunk_length = [0u8; 4];
            file.read_exact(&mut raw_chunk_length).expect("Failed to read chunk length");
            let mut raw_compression_type = [0u8; 1];
            file.read_exact(&mut raw_compression_type).expect("Failed to read compression type");

            let chunk_length = u32::from_be_bytes(raw_chunk_length);
            let compression_type = u8::from_be_bytes(raw_compression_type);

            let mut raw_chunk_data = vec![0u8; (chunk_length-1) as usize];
            file.read_exact(&mut raw_chunk_data).expect("Failed to read chunk data");
            let chunk = self.parse_chunk(raw_chunk_data, compression_type, region.position.dimension());
            if chunk.is_some() {
                chunks.push(chunk.unwrap());
            }
        }
        //

        chunks
    }

    fn parse_chunk(&self, data: Vec<u8>, compression_type: u8, dimension: &str) -> Option<Chunk<'a>> {
        let mut chunk_data = data;

        match compression_type {
            2 => { chunk_data = self.uncompress_zlib(chunk_data); }
            _ => { return None }
        }
        let chunk_nbt: NBTChunk = fastnbt::from_bytes(chunk_data.as_slice()).expect("Failed to parse chunk data");

        let mut chunk = Chunk::new(
            self.version,
            Position::new(dimension, chunk_nbt.x_pos as f32, 0f32, chunk_nbt.z_pos as f32),
            chunk_nbt.data_version,
            Tick::new(chunk_nbt.inhabited_time as usize),
            Tick::new(chunk_nbt.last_update as usize),
            chunk_nbt.status.clone()
        );

        unsafe { self.populate_chunk(&mut chunk, chunk_nbt); }

        Some(chunk)
    }
}


