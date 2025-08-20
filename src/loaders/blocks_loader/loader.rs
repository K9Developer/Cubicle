use super::v3465::BlockLoaderV3465;
use crate::constants::versions::Version;
use crate::models::other::region::Region;
use crate::models::world::chunk::Chunk;
use std::path::PathBuf;

// TODO: Add more funcs and lazy loading like empty_load all regions so we get metadata of all chunks and we can count them, etc. - Think of a system later

pub trait BlockLoader<'a> {
    fn get_region_files(&self, world_path: PathBuf) -> Vec<Region>;
    fn parse_region(&self, region: &Region) -> Vec<Chunk<'a>>;
    fn parse_chunk(
        &self,
        data: Vec<u8>,
        compression_type: u8,
        dimension: &str,
    ) -> Option<Chunk<'a>>;
}

pub fn get_block_loader(version: &Version) -> Box<dyn BlockLoader + '_> {
    match version.data.version_data {
        3465 => Box::new(BlockLoaderV3465 { version }),
        _ => panic!(),
    }
}
