use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use crate::constants::versions::Version;
use crate::loaders::entities_loader::v3465::EntityLoaderV3465;
use crate::models::entity::entity::Entity;
use crate::models::other::region::Region;

// TODO: Add more funcs and lazy loading like empty_load all regions so we get metadata of all chunks and we can count them, etc. - Think of a system later

pub trait EntityLoader<'a> {
    fn get_region_files(&self, world_path: PathBuf) -> Vec<Region>;
    fn parse_region(&self, region: &Region) -> HashMap<(i32, i32), Vec<Entity>>;
    fn parse_entity_chunk(&self, data: Vec<u8>, compression_type: u8, dimension: &str) -> Option<Vec<Entity>>;
}

pub fn get_entity_loader<'a>(version: Arc<Version>) -> Box<dyn EntityLoader<'a>> {
    match version.data.version_data {
        3465 => Box::new( EntityLoaderV3465 { version }),
        _ => panic!()
    }
}