use std::collections::HashMap;
use crate::constants::versions::Version;
use crate::models::other::region::Region;
use crate::models::world::chunk::Chunk;
use std::path::PathBuf;
use std::sync::Arc;
use crate::loaders::v3465::block_loader::BlockLoaderV3465;
use crate::models::other::lasso_string::LassoString;
use crate::models::world_structures::generic_structure::GenericParentStructure;
// TODO: Add more funcs and lazy loading like empty_load all regions so we get metadata of all chunks and we can count them, etc. - Think of a system later

pub trait BlockLoader<'a> {
    fn get_region_files(&self, world_path: PathBuf) -> Vec<Region>;
    fn parse_region(&self, region: &Region) -> (Vec<Chunk>, HashMap<i64, Vec<GenericParentStructure>>);
    fn parse_chunk(
        &self,
        data: Vec<u8>,
        compression_type: u8,
        dimension: &LassoString,
    ) -> Option<(Chunk, Vec<GenericParentStructure>)>;
}

pub fn get_block_loader<'a>(version: Arc<Version>) -> Box<dyn BlockLoader<'a>> {
    match version.data.version_data {
        3465 => Box::new(BlockLoaderV3465 { version }),
        _ => panic!(),
    }
}
