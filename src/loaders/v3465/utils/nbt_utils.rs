use std::collections::HashMap;
use fastnbt::Value;
use crate::loaders::v3465::utils::entity_utils::{parse_nbt_entity, parse_raw_nbt_entity};
use crate::models::block_entity::block_entity::GenericBlockEntity;
use crate::models::block_entity::prelude::*;
use crate::models::other::inventory::{Inventory, Item};
use crate::models::other::mojang_data::text_component::TextComponent;
use crate::models::other::properties::Properties;
use crate::models::block_entity::types::storage_container::types::standard_container::StandardStorageContainerBlockEntity;
use crate::models::entity::entity::{Entity, MobEntity};
use crate::models::other::lasso_string::LassoString;
use crate::models::other::mojang_data::color::MinecraftColor;

pub fn parse_nbt_item(mut map: Option<HashMap<String, Value>>) -> Option<(usize, Item)> {
    if let Some(mut map) = map {
        if let (Some(Value::String(item_id)),
            Some(Value::Byte(slot)),
            Some(Value::Byte(count))) =
            (map.remove("id"), map.remove("Slot"), map.remove("Count"))
        {
            return Some((slot as usize, Item::new(item_id, count as u32, Properties::new(map))))
        }
    }
    None
}

pub fn parse_nbt_storage_container(base: GenericBlockEntity, map: &mut HashMap<String, Value>, size: usize) -> StandardStorageContainerBlockEntity {
    StandardStorageContainerBlockEntity::new(
        base,
        take_string(map, "Lock"),
        take_text_component(map, "CustomName"),
        take_inventory(map, "Items", size)
    )
}

fn parse_nbt_range(val: Value) -> Option<LightLimitRange> {
    match val {
        Value::Int(block_light_lim) => {
            Some(LightLimitRange::new_value(block_light_lim.clone()))
        },
        Value::Compound(block_light_lim) => {
            if let Ok(l) = fastnbt::from_value(&Value::Compound(block_light_lim)) {l}
            else { None }
        },
        _ => None
    }
}

pub fn parse_nbt_spawn_rules(map: Option<HashMap<String, Value>>) -> Option<CustomSpawnRules> {
    match map {
        None => {None},
        Some(mut map) => {
            let block_light_lim = map.remove("block_light_limit")?;
            let mut block_light_range: LightLimitRange = parse_nbt_range(block_light_lim)?;

            let sky_light_lim = map.remove("sky_light_limit")?;
            let mut sky_light_range: LightLimitRange = parse_nbt_range(sky_light_lim)?;

            Some(CustomSpawnRules {
                block_light_range,
                sky_light_range,
            })
        }
    }
}

pub fn parse_nbt_spawn_equipment(map: Option<HashMap<String, Value>>) -> Option<SpawnEquipment> {

    match map {
        None => { None }
        Some(mut map) => {
            let loot_table = {
                if let Value::String(l) = map.remove("loot_table")? { l }
                else { return None }
            };


            let slot_drop_chances: EquipmentDropChances = {
                let sdc = map.remove("slot_drop_chances");
                if let Some(Value::Compound(slot_drop_chances)) = sdc {
                    match fastnbt::from_value(&Value::Compound(slot_drop_chances)) {
                        Ok(slot_drop_chances) => {slot_drop_chances},
                        Err(_) => {return None}
                    }
                } else if let Some(Value::Float(slot_drop_chances)) = sdc {
                    EquipmentDropChances::new_uniform(slot_drop_chances)
                }
                else { return None }
            };

            Some(SpawnEquipment::new(loot_table, slot_drop_chances))
        }
    }
}

pub fn parse_spawner_spawn_data(map: Option<HashMap<String, Value>>, dimension: &LassoString) -> SpawnerSpawnData {
    if let Some(mut map) = map {
        let entity = parse_raw_nbt_entity(take_map(&mut map, "entity"), dimension);
        if let Some(entity) = entity {
            return SpawnerSpawnData::new(
                entity,
                parse_nbt_spawn_rules(take_map(&mut map, "custom_spawn_rules")),
                parse_nbt_spawn_equipment(take_map(&mut map, "equipment")),
            )
        }
    }
    SpawnerSpawnData::new(Entity::default(), None, None)
}

pub fn parse_spawner_spawn_potentials(list: Option<Vec<Value>>, dimension: &LassoString) -> Option<Vec<SpawnPotential>> {
    if let Some(list) = list {
        let mut pot_vec = Vec::new();
        for nbt_pot in list {
            if let Value::Compound(mut map) = nbt_pot {
                let weight = take_i32(&mut map, "weight")?;
                let spawn_data = parse_spawner_spawn_data(take_map(&mut map, "data"), dimension);
                pot_vec.push(
                    SpawnPotential::new(
                        weight,
                        spawn_data
                    )
                )
            }
        }
        return if pot_vec.len() == 0 { None } else { Some(pot_vec) }
    }
    None
}

pub fn parse_sign_text_block(map: Option<HashMap<String, Value>>) -> SignTextBlock {
    if let Some(mut map) = map {
        return SignTextBlock::new(
            take_bool(&mut map, "has_glowing_text").unwrap_or(false),
            MinecraftColor::from(take_string(&mut map, "color").unwrap_or_default()),
            take_string_list(&mut map, "messages").unwrap_or(Vec::new()).iter().map(
                |s| TextComponent::from_string(s)
            ).collect(),
        )
    }
    SignTextBlock::new(false, MinecraftColor::Black, Vec::new())
}

// NBT Utils
pub fn take_string(map: &mut HashMap<String, Value>, key: &str) -> Option<String> {
    map.remove(key).and_then(|v| match v {
        Value::String(s) => Some(s),
        _ => None
    })
}
pub fn take_i8(map: &mut HashMap<String, Value>, key: &str) -> Option<i8> {
    map.remove(key).and_then(|v| match v {
        Value::Byte(s) => Some(s),
        _ => None
    })
}
pub fn take_i16(map: &mut HashMap<String, Value>, key: &str) -> Option<i16> {
    map.remove(key).and_then(|v| match v {
        Value::Short(s) => Some(s),
        _ => None
    })
}
pub fn take_i32(map: &mut HashMap<String, Value>, key: &str) -> Option<i32> {
    map.remove(key).and_then(|v| match v {
        Value::Int(s) => Some(s),
        _ => None
    })
}
pub fn take_bool(map: &mut HashMap<String, Value>, key: &str) -> Option<bool> {
    map.remove(key).and_then(|v| match v {
        Value::Byte(s) => Some(s != 0),
        _ => None
    })
}
pub fn take_long(map: &mut HashMap<String, Value>, key: &str) -> Option<i64> {
    map.remove(key).and_then(|v| match v {
        Value::Long(s) => Some(s),
        _ => None
    })
}
pub fn take_i32_vec(map: &mut HashMap<String, Value>, key: &str) -> Option<Vec<i32>> {
    map.remove(key).and_then(|v| match v {
        Value::IntArray(ia) => Some(ia.to_vec()),
        _ => None
    })
}
pub fn take_list(map: &mut HashMap<String, Value>, key: &str) -> Option<Vec<Value>> {
    map.remove(key).and_then(|v| match v {
        Value::List(ia) => Some(ia),
        _ => None
    })
}
pub fn take_text_component(map: &mut HashMap<String, Value>, key: &str) -> Option<TextComponent> {
    take_string(map, key).and_then(|s| Some(TextComponent::from_string(&s)))
}

pub fn take_map<V>(map: &mut HashMap<String, Value>, key: &str) -> Option<HashMap<String, V>>
where
    Value: TryInto<V>,
{
    map.remove(key).and_then(|v| match v {
        Value::Compound(inner) => {
            let mut out = HashMap::with_capacity(inner.len());
            for (k, val) in inner {
                if let Ok(v) = val.try_into() {
                    out.insert(k, v);
                }
            }
            Some(out)
        }
        _ => None,
    })
}
pub fn take_string_list(map: &mut HashMap<String, Value>, key: &str) -> Option<Vec<String>> {
    map.remove(key).and_then(|v| match v {
        Value::List(inner) => {
            let mut out = Vec::with_capacity(inner.len());
            for val in inner {
               if let Value::String(s) = val {
                   out.push(s);
               } else {return None}
            }
            Some(out)
        }
        _ => None,
    })
}
pub fn take_inventory(map: &mut HashMap<String, Value>, key: &str, size: usize) -> Inventory {
    let items = map.remove(key);
    let mut inv = Inventory::new(size);

    if let Some(Value::List(il)) = items {
        for nbt_value in il {
            if let Value::Compound(nbt_item) = nbt_value {
                if let Some((slot, item)) = parse_nbt_item(Some(nbt_item)) {
                    inv.set_item(slot, item);
                }
            }
        }
    }

    inv
}