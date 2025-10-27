use serde::{Deserialize, Serialize};
use crate::models::block_entity::block_entity::GenericBlockEntity;
use crate::models::entity::entity::Entity;
use crate::models::other::tick::Tick;
use crate::traits::block_entity::BlockEntityTrait;

#[derive(Debug, Deserialize, Serialize)]
pub struct LightLimitRange {
    pub min_inclusive: i32,
    pub max_inclusive: i32,
}

impl LightLimitRange {
    pub fn new(min_inclusive: i32, max_inclusive: i32) -> LightLimitRange {
        LightLimitRange {min_inclusive, max_inclusive}
    }

    pub fn new_value(val: i32) -> LightLimitRange {
        LightLimitRange {min_inclusive: 0, max_inclusive: val}
    }
}

#[derive(Debug)]
pub struct CustomSpawnRules {
    pub block_light_range: LightLimitRange,
    pub sky_light_range: LightLimitRange,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EquipmentDropChances {
    pub feet: f32,
    pub legs: f32,
    pub chest: f32,
    pub head: f32,
    pub body: f32,
    pub mainhand: f32,
    pub offhand: f32
}

impl EquipmentDropChances {
    pub fn new_uniform(val: f32) -> Self {
        EquipmentDropChances {
            feet: val,
            legs: val,
            chest: val,
            head: val,
            body: val,
            mainhand: val,
            offhand: val
        }
    }
}

#[derive(Debug)]
pub struct SpawnEquipment {
    loot_table: String, // TODO: Load the loot table and have it as a ref in a generalized structure
    drop_chances: EquipmentDropChances
}

impl SpawnEquipment {
    pub fn new(loot_table_resource: String, drop_chances: EquipmentDropChances) -> Self {
        SpawnEquipment {
            loot_table: loot_table_resource,
            drop_chances
        }
    }

    pub fn loot_table(&self) -> &str { &self.loot_table }
    pub fn drop_chances(&self) -> &EquipmentDropChances { &self.drop_chances }

    pub fn set_loot_table(&mut self, loot_table: String) { self.loot_table = loot_table; }
    pub fn set_drop_chances(&mut self, drop_chances: EquipmentDropChances) { self.drop_chances = drop_chances; }
}

#[derive(Debug)]
pub struct SpawnerSpawnData {
    entity: Entity,
    spawn_rules: Option<CustomSpawnRules>,
    equipment: Option<SpawnEquipment>
}

impl SpawnerSpawnData {
    pub fn new(entity: Entity, spawn_rules: Option<CustomSpawnRules>, equipment: Option<SpawnEquipment>) -> Self {
        SpawnerSpawnData {
            entity, spawn_rules, equipment
        }
    }

    pub fn entity(&self) -> &Entity { &self.entity }
    pub fn spawn_rules(&self) -> &Option<CustomSpawnRules> { &self.spawn_rules }
    pub fn equipment(&self) -> &Option<SpawnEquipment> { &self.equipment }

    pub fn set_entity(&mut self, entity: Entity) { self.entity = entity; }
    pub fn set_spawn_rules(&mut self, spawn_rules: Option<CustomSpawnRules>) { self.spawn_rules = spawn_rules; }
    pub fn set_equipment(&mut self, equipment: Option<SpawnEquipment>) { self.equipment = equipment; }
}

#[derive(Debug)]
pub struct SpawnPotential {
    weight: i32,
    data: SpawnerSpawnData
}

impl SpawnPotential {
    pub fn new(weight: i32, data: SpawnerSpawnData) -> Self {
        SpawnPotential { weight, data }
    }
}

#[derive(Debug)]
pub struct SpawnerBlockEntity {
    base: GenericBlockEntity,

    ticks_until_next_spawn: Tick,
    max_nearby_entities: i16,
    max_spawn_delay: Tick,
    min_spawn_delay: Tick,
    required_player_range: i16,
    spawn_batch_size: i16,
    spawn_range: i16,
    next_spawn_data: SpawnerSpawnData,
    spawn_potentials: Vec<SpawnPotential>,
}

impl SpawnerBlockEntity {
    pub fn new(
        base: GenericBlockEntity,
        ticks_until_next_spawn: Tick,
        max_nearby_entities: i16,
        max_spawn_delay: Tick,
        min_spawn_delay: Tick,
        required_player_range: i16,
        spawn_batch_size: i16,
        spawn_range: i16,
        next_spawn_data: SpawnerSpawnData,
        spawn_potentials: Vec<SpawnPotential>,
    ) -> Self {
        SpawnerBlockEntity {
            base,
            ticks_until_next_spawn,
            max_nearby_entities,
            max_spawn_delay,
            min_spawn_delay,
            required_player_range,
            spawn_batch_size,
            spawn_range,
            next_spawn_data,
            spawn_potentials
        }
    }

    pub fn ticks_until_next_spawn(&self) -> &Tick { &self.ticks_until_next_spawn }
    pub fn max_nearby_entities(&self) -> i16 { self.max_nearby_entities }
    pub fn max_spawn_delay(&self) -> Tick { self.max_spawn_delay }
    pub fn min_spawn_delay(&self) -> Tick { self.min_spawn_delay }
    pub fn required_player_range(&self) -> i16 { self.required_player_range }
    pub fn spawn_batch_size(&self) -> i16 { self.spawn_batch_size }
    pub fn spawn_range(&self) -> i16 { self.spawn_range }
    pub fn next_spawn_data(&self) -> &SpawnerSpawnData { &self.next_spawn_data }
    pub fn spawn_potentials(&self) -> &Vec<SpawnPotential> { &self.spawn_potentials }
    pub fn potential_count(self) -> usize { self.spawn_potentials.len() }

    pub fn set_ticks_until_next_spawn(&mut self, tick: Tick) { self.ticks_until_next_spawn = tick; }
    pub fn set_max_nearby_entities(&mut self, max_nearby_entities: i16) { self.max_nearby_entities = max_nearby_entities; }
    pub fn set_max_spawn_delay(&mut self, max_delay: Tick) { self.max_spawn_delay = max_delay; }
    pub fn set_min_spawn_delay(&mut self, min_delay: Tick) { self.min_spawn_delay = min_delay; }
    pub fn set_required_player_range(&mut self, required_player_range: i16) { self.required_player_range = required_player_range; }
    pub fn set_spawn_batch_size(&mut self, spawn_batch_size: i16) { self.spawn_batch_size = spawn_batch_size; }
    pub fn set_spawn_range(&mut self, spawn_range: i16) { self.spawn_range = spawn_range; }
    pub fn set_next_spawn_data(&mut self, spawn_data: SpawnerSpawnData) { self.next_spawn_data = spawn_data; }
    pub fn add_potential(&mut self, spawn_potential: SpawnPotential) { self.spawn_potentials.push(spawn_potential); }
    pub fn remove_potential(&mut self, ind: usize) { self.spawn_potentials.remove(ind); }
    pub fn clear_potentials(&mut self) { self.spawn_potentials.clear(); }
}

impl BlockEntityTrait for SpawnerBlockEntity {
    fn base(&self) -> &GenericBlockEntity {
        &self.base
    }
    fn base_mut(&mut self) -> &mut GenericBlockEntity { &mut self.base }
}