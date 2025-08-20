use std::path::PathBuf;
use super::loader::{EntityLoader};
use crate::models::other::region::Region;
use crate::models::nbt_structures::v3465::entities::{NBTChunk};
use crate::constants::constants::{ZLIB_COMPRESSION_TYPE};
use crate::constants::versions::Version;
use crate::loaders::utils::{get_region_files_in_folder, parse_region_file, uncompress_zlib};
use crate::models::entity::entity::Entity;
use crate::models::world::world::WorldType;
// TODO: Support other dimensions (custom paths)

pub(super) struct EntityLoaderV3465<'a> {
    pub(crate) version: &'a Version
}

impl<'a> EntityLoaderV3465<'a> {
    fn populate_entity_list(&self, entity_list: &mut Vec<Entity>, chunk_nbt: NBTChunk, dimension: &str) {
        //TODO
    }
}


impl<'a> EntityLoader<'a> for EntityLoaderV3465<'a> {
    // TODO: I dont really like the fact that paths are duplicates in all loaders, etc.
    fn get_region_files(&self, world_path: PathBuf) -> Vec<Region> {
        let overworld_region_folder = world_path.join((if self.version.world_type() == &WorldType::MULTIPLAYER {"world/"} else {""}).to_owned() + "entities");
        let nether_region_folder = world_path.join((if self.version.world_type() == &WorldType::MULTIPLAYER {"world/"} else {""}).to_owned() + "DIM-1/entities");
        let end_region_folder = world_path.join((if self.version.world_type() == &WorldType::MULTIPLAYER {"world/"} else {""}).to_owned() + "DIM1/entities");

        let mut regions = Vec::<Region>::new();

        regions.extend(get_region_files_in_folder(&overworld_region_folder, "overworld"));
        regions.extend(get_region_files_in_folder(&nether_region_folder, "the_nether"));
        regions.extend(get_region_files_in_folder(&end_region_folder, "the_end"));

        regions
    }

    fn parse_region(&self, region: &Region) -> Vec<Entity> {
        let parsed_chunks = parse_region_file(region);
        let mut entities = Vec::with_capacity(parsed_chunks.len());
        for parsed_chunk in parsed_chunks {
            let chunk = self.parse_entity_chunk(parsed_chunk.raw_bytes, parsed_chunk.compression_type, region.position.dimension());
            if chunk.is_some() {
                entities.extend(chunk.unwrap());
            }
        }
        entities
    }

    fn parse_entity_chunk(&self, data: Vec<u8>, compression_type: u8, dimension: &str) -> Option<Vec<Entity>> {
        let mut chunk_data = data;

        match compression_type {
            ZLIB_COMPRESSION_TYPE => { chunk_data = uncompress_zlib(chunk_data) }
            _ => { return None }
        }

        let chunk_nbt: NBTChunk = fastnbt::from_bytes(chunk_data.as_slice()).expect("Failed to parse chunk data");

        let mut entities = Vec::<Entity>::with_capacity(chunk_nbt.entities.len());

        self.populate_entity_list(&mut entities, chunk_nbt, dimension);
        Some(entities)
    }
}


