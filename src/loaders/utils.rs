use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use flate2::read::ZlibDecoder;
use crate::constants::constants::{MCA_REGION_LOCATION_SECTOR_ENTRY_SIZE, MCA_REGION_SECTOR_SIZE, ZLIB_COMPRESSION_TYPE};
use crate::models::other::region::{Region, RegionType};
use crate::types::RegionPosition;

pub struct ParsedRegionChunk {
    pub offset: usize,
    pub header_offset: usize,
    pub raw_bytes: Vec<u8>,
    pub compression_type: u8
}

pub fn get_region_files_in_folder(folder: &PathBuf, dimension_name: &str, region_type: RegionType) -> Vec<Region> {
    if !folder.exists() {
        println!("{} does not exist, skipping...", folder.display());
        return Vec::new();
    }

    let mut regions = Vec::<Region>::new();

    for file in folder.read_dir().expect("Failed to read folder") {
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
            position: RegionPosition::new(region_x.unwrap(), region_z.unwrap(), dimension_name),
            path: file.path().to_path_buf(),
            region_type: region_type.clone() // bad maybe?
        })
    }

    regions
}

pub fn parse_region_file(region: &Region) -> Vec<ParsedRegionChunk> {
    let mut file = File::open(region.path.clone()).expect("Failed to open region file");

    let mut loc_table = [0u8; MCA_REGION_SECTOR_SIZE];
    file.read_exact(&mut loc_table).expect("Failed to read locations table");
    let mut offsets = Vec::<(usize, usize)>::with_capacity(MCA_REGION_SECTOR_SIZE/MCA_REGION_LOCATION_SECTOR_ENTRY_SIZE);

    for i in 0..(MCA_REGION_SECTOR_SIZE/MCA_REGION_LOCATION_SECTOR_ENTRY_SIZE) {
        let b0 = loc_table[i * 4];
        let b1 = loc_table[i * 4 + 1];
        let b2 = loc_table[i * 4 + 2];
        let b3 = loc_table[i * 4 + 3]; // sector count

        let entry = u32::from_be_bytes([b0, b1, b2, b3]);
        let sector_offset = (entry >> 8) as usize; // top 24 bits
        let sector_count  = (entry & 0xFF) as u8;
        if sector_offset != 0 && sector_count != 0 { offsets.push((sector_offset * MCA_REGION_SECTOR_SIZE, i*4)); }
    }

    let mut parsed_chunks = Vec::<ParsedRegionChunk>::new();
    for (offset, loc_offset) in offsets {
        file.seek(SeekFrom::Start(offset as u64)).expect("Failed to seek");

        let mut raw_chunk_length = [0u8; 4];
        file.read_exact(&mut raw_chunk_length).expect("Failed to read chunk length");
        let mut raw_compression_type = [0u8; 1];
        file.read_exact(&mut raw_compression_type).expect("Failed to read compression type");

        let chunk_length = u32::from_be_bytes(raw_chunk_length);
        let compression_type = u8::from_be_bytes(raw_compression_type);

        let mut raw_chunk_data = vec![0u8; (chunk_length-1) as usize];
        file.read_exact(&mut raw_chunk_data).expect("Failed to read chunk data");
        parsed_chunks.push(ParsedRegionChunk {
            offset,
            header_offset: loc_offset,
            raw_bytes: raw_chunk_data,
            compression_type
        })
    }

    parsed_chunks
}

pub fn uncompress_zlib(data: Vec<u8>) -> Option<Vec<u8>> {
    fn guess_output_capacity(compressed_len: usize) -> usize {
        const MIN_START: usize = 256 * 1024;
        const FACTOR_NUM: usize = 4;
        const FACTOR_DEN: usize = 1;
        const MAX_START: usize = 32 * 1024 * 1024;
        let guess = compressed_len.saturating_mul(FACTOR_NUM) / FACTOR_DEN;
        guess.clamp(MIN_START, MAX_START)
    }

    let mut decoder = ZlibDecoder::new(&data[..]);
    let mut decompressed = Vec::with_capacity(guess_output_capacity(data.len()));
    match decoder.read_to_end(&mut decompressed) {
        Ok(_) => { decompressed.shrink_to_fit() },
        Err(_) => { return None }
    }

    Some(decompressed)
}

pub fn handle_chunk_compression(compression_type: u8, chunk_data: Vec<u8>) -> Option<Vec<u8>> {
    match compression_type {
        ZLIB_COMPRESSION_TYPE => {
            match uncompress_zlib(chunk_data) {
                Some(c) => Some(c),
                None => { None }
            }
        }
        _ => { todo!() }
    }
}

#[inline(always)]
pub fn nbt_uuid_to_u128(data: [i32; 4]) -> u128 {
    ((data[0] as u32 as u128) << 96) |
        ((data[1] as u32 as u128) << 64) |
        ((data[2] as u32 as u128) << 32) |
        (data[3] as u32 as u128)
}