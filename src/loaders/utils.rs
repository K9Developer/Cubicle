use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use flate2::read::ZlibDecoder;
use crate::constants::constants::{MCA_REGION_LOCATION_SECTOR_ENTRY_SIZE, MCA_REGION_SECTOR_SIZE};
use crate::models::other::position::Position;
use crate::models::other::region::{Region, RegionType};

pub struct ParsedRegionChunk {
    pub offset: usize,
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
            position: Position::new(dimension_name, region_x.unwrap(), 0, region_z.unwrap()),
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

    let mut parsed_chunks = Vec::<ParsedRegionChunk>::new();
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
        parsed_chunks.push(ParsedRegionChunk {
            offset,
            raw_bytes: raw_chunk_data,
            compression_type
        })
    }

    parsed_chunks
}

pub fn uncompress_zlib(data: Vec<u8>) -> Vec<u8> {
    let mut decoder = ZlibDecoder::new(&data[..]);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).unwrap();
    decompressed
}

#[inline(always)]
pub fn nbt_uuid_to_u128(data: [i32; 4]) -> u128 {
    ((data[0] as u32 as u128) << 96) |
        ((data[1] as u32 as u128) << 64) |
        ((data[2] as u32 as u128) << 32) |
        (data[3] as u32 as u128)
}