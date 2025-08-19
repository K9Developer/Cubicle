use std::path::PathBuf;
use crate::constants::versions::Version;
use crate::models::world::chunk::Chunk;
use crate::loaders::v1_20_1::WorldLoaderV012001;
use crate::models::other::region::Region;

// TODO: Add more funcs and lazy loading like empty_load all regions so we get metadata of all chunks and we can count them, etc. - Think of a system later

pub trait Loader<'a> {
    fn get_region_files(&self, world_path: PathBuf) -> Vec<Region>;
    fn parse_region(&self, region: &Region) -> Vec<Chunk<'a>>;
    fn parse_chunk(&self, data: Vec<u8>, compression_type: u8, dimension: &str) -> Option<Chunk<'a>>;
}

pub fn get_loader(version: &Version) -> Box<dyn Loader + '_> {
    match version.to_string().as_str() {
        "1.20.1" => Box::new(WorldLoaderV012001 { version }),
        _ => panic!()
    }
}