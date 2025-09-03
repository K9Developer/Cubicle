use std::collections::HashMap;
use std::hash::Hash;
use std::path::PathBuf;
use std::sync::Arc;
use fastnbt::Value;
use super::loader::{EntityLoader};
use crate::models::other::region::{Region, RegionType};
use crate::models::nbt_structures::v3465::entities::{NBTChunk};
use crate::constants::constants::{ZLIB_COMPRESSION_TYPE};
use crate::constants::versions::Version;
use crate::loaders::utils::{get_region_files_in_folder, nbt_uuid_to_u128, parse_region_file, uncompress_zlib};
use crate::models::entity::entity::{Entity, EntityType, MobEntity};
use crate::models::other::properties::Properties;
use crate::models::other::tick::Tick;
use crate::models::positions::entity_position::EntityPosition;
use crate::models::world::world::WorldKind;
// TODO: Support other dimensions (custom paths)

pub(super) struct EntityLoaderV3465 {
    pub(crate) version: Arc<Version>
}

impl EntityLoaderV3465 {
    fn populate_entity_list(&self, entity_list: &mut Vec<Entity>, chunk_nbt: NBTChunk, dimension: &str) {
        for entity in chunk_nbt.entities {
            if let Value::IntArray(arr) = entity.uuid {
                let uuid_parts: &[i32] = &*arr;
                let e = MobEntity::new(
                    entity.id,
                    Tick::new(entity.air_left as usize),
                    entity.distance_fallen,
                    Tick::new(entity.fire_ticks_left as usize),
                    entity.is_invulnerable,
                    <(f64, f64, f64)>::from(entity.motion),
                    entity.is_on_ground,
                    EntityPosition::new(entity.position[0], entity.position[1], entity.position[2], entity.rotation[0], entity.rotation[1], dimension),
                    nbt_uuid_to_u128(<[i32; 4]>::try_from(uuid_parts).unwrap()), // TODO: maybe this is slow too.
                    Properties::new(entity.others)
                );
                entity_list.push(Entity::Mob(e));
            }
        }
    }
}


impl<'a> EntityLoader<'a> for EntityLoaderV3465 {
    // TODO: I dont really like the fact that paths are duplicates in all loaders, etc.
    fn get_region_files(&self, world_path: PathBuf) -> Vec<Region> {
        let overworld_region_folder = world_path.join((if self.version.world_type() == &WorldKind::MULTIPLAYER {"world/"} else {""}).to_owned() + "entities");
        let nether_region_folder = world_path.join((if self.version.world_type() == &WorldKind::MULTIPLAYER {"world/"} else {""}).to_owned() + "DIM-1/entities");
        let end_region_folder = world_path.join((if self.version.world_type() == &WorldKind::MULTIPLAYER {"world/"} else {""}).to_owned() + "DIM1/entities");

        let mut regions = Vec::<Region>::new();

        regions.extend(get_region_files_in_folder(&overworld_region_folder, "overworld", RegionType::Entity));
        regions.extend(get_region_files_in_folder(&nether_region_folder, "the_nether", RegionType::Entity));
        regions.extend(get_region_files_in_folder(&end_region_folder, "the_end", RegionType::Entity));

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


