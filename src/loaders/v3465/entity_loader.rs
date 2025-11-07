use std::collections::HashMap;
use std::hash::Hash;
use std::path::PathBuf;
use std::sync::Arc;
use crate::models::other::region::{Region, RegionType};
use crate::models::nbt_structures::v3465::entities::{NBTChunk};
use crate::constants::versions::Version;
use crate::loaders::loader_utils::{get_region_files_in_folder, handle_chunk_compression, nbt_uuid_to_u128, parse_region_file, uncompress_zlib};
use crate::loaders::templates::entity_loader::EntityLoader;
use crate::loaders::v3465::utils::entity_utils::parse_nbt_entity;
use crate::models::entity::entity::{Entity, EntityType, MobEntity};
use crate::models::other::lasso_string::LassoString;
use crate::types::WorldKind;
use crate::utils::position_utils::chunk_offset_to_position;
// TODO: Support other dimensions (custom paths)

pub struct EntityLoaderV3465 {
    pub version: Arc<Version>
}

impl EntityLoaderV3465 {
    fn populate_entity_list(&self, entity_list: &mut Vec<Entity>, chunk_nbt: NBTChunk, dimension: &LassoString) {
        for entity in chunk_nbt.entities {
            if let Some(ent) = parse_nbt_entity(entity, dimension) {
                entity_list.push(ent);
            }
        }
    }
}


impl<'a> EntityLoader<'a> for EntityLoaderV3465 {
    // TODO: I dont really like the fact that paths are duplicates in all loaders, etc.
    fn get_region_files(&self, world_path: PathBuf) -> Vec<Region> {
        let overworld_region_folder = world_path.join((if self.version.world_type() == &WorldKind::Multiplayer {"world/"} else {""}).to_owned() + "entities");
        let nether_region_folder = world_path.join((if self.version.world_type() == &WorldKind::Multiplayer {"world/"} else {""}).to_owned() + "DIM-1/entities");
        let end_region_folder = world_path.join((if self.version.world_type() == &WorldKind::Multiplayer {"world/"} else {""}).to_owned() + "DIM1/entities");

        let mut regions = Vec::<Region>::new();

        regions.extend(get_region_files_in_folder(&overworld_region_folder, "overworld".into(), RegionType::Entity));
        regions.extend(get_region_files_in_folder(&nether_region_folder, "the_nether".into(), RegionType::Entity));
        regions.extend(get_region_files_in_folder(&end_region_folder, "the_end".into(), RegionType::Entity));

        regions
    }

    fn parse_region(&self, region: &Region) -> HashMap<(i32, i32), Vec<Entity>> {
        let parsed_chunks = parse_region_file(region);
        let mut entities = HashMap::new();
        for parsed_chunk in parsed_chunks {
            let chunk_entities = self.parse_entity_chunk(parsed_chunk.raw_bytes, parsed_chunk.compression_type, region.position.dimension());
            if chunk_entities.is_some() {
                let chunk_pos = chunk_offset_to_position(parsed_chunk.header_offset, region);
                entities.insert(chunk_pos, chunk_entities.unwrap());
            }
        }
        entities
    }

    fn parse_entity_chunk(&self, data: Vec<u8>, compression_type: u8, dimension: &LassoString) -> Option<Vec<Entity>> {
        let chunk_data = handle_chunk_compression(compression_type, data)?;
        let chunk_nbt: NBTChunk = fastnbt::from_bytes(chunk_data.as_slice()).expect("Failed to parse chunk data");

        let mut entities = Vec::<Entity>::with_capacity(chunk_nbt.entities.len());

        self.populate_entity_list(&mut entities, chunk_nbt, dimension);
        Some(entities)
    }
}


