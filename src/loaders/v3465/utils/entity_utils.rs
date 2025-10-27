use std::collections::HashMap;
use fastnbt::Value;
use crate::loaders::loader_utils::nbt_uuid_to_u128;
use crate::models::entity::entity::{Entity, MobEntity};
use crate::models::nbt_structures::v3465::entities::NBTEntity;
use crate::models::other::lasso_string::LassoString;
use crate::models::other::properties::Properties;
use crate::models::other::tick::Tick;
use crate::models::positions::entity_position::EntityPosition;

pub fn parse_nbt_entity(nbt_entity: NBTEntity, dimension: &LassoString) -> Option<Entity> {
    if let Value::IntArray(arr) = nbt_entity.uuid {
        let uuid_parts: &[i32] = &*arr;
        let e = MobEntity::new(
            nbt_entity.id,
            Tick::new(nbt_entity.air_left as usize),
            nbt_entity.distance_fallen,
            Tick::new(nbt_entity.fire_ticks_left as usize),
            nbt_entity.is_invulnerable,
            <(f64, f64, f64)>::from(nbt_entity.motion),
            nbt_entity.is_on_ground,
            EntityPosition::new(nbt_entity.position[0], nbt_entity.position[1], nbt_entity.position[2], nbt_entity.rotation[0], nbt_entity.rotation[1], dimension.clone()),
            nbt_uuid_to_u128(<[i32; 4]>::try_from(uuid_parts).unwrap()),
            Properties::new(nbt_entity.others)
        );
        return Some(Entity::Mob(e))
    }
    None
}

pub fn parse_raw_nbt_entity(raw: Option<HashMap<String, Value>>, dimension: &LassoString) -> Option<Entity> {
    if let Some(map) = raw {
        let nbt_entity = { fastnbt::from_value(&Value::Compound(map)) };
        if let Ok(nbt_entity) = nbt_entity {
            return parse_nbt_entity(nbt_entity, dimension)
        }
    }
    None
}