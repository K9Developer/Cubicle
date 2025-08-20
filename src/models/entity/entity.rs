use std::collections::HashMap;
use fastnbt::Value;
use crate::models::other::inventory::Inventory;
use crate::models::other::position::{EntityPosition};
use crate::models::other::tick::Tick;

// TODO: Actually complete the Entities

#[derive(Debug)]
struct GenericEntity {
    air_remaining: Tick,
    distance_fallen: f32,
    fire_ticks_left: Tick,
    is_invulnerable: bool,
    velocity_vector: (f32, f32, f32),
    is_on_ground: bool,
    position: EntityPosition,
    uuid: u128
}

pub enum EntityType {
    PLAYER,
    MOB
}

#[derive(Debug)]
struct PlayerEntity { // TODO
    base: GenericEntity,
    inventory: Inventory
}

#[derive(Debug)]
struct MobEntity {
    base: GenericEntity,
    id: String,
    extra: HashMap<String, Value>
}

#[derive(Debug)]
pub enum Entity {
    Player(PlayerEntity),
    Mob(MobEntity),
}

impl Entity {
    fn new(entity_type: EntityType, air_remaining: Tick, distance_fallen: f32, fire_ticks_left: Tick, is_invulnerable: bool, velocity_vector: (f32,f32,f32), is_on_ground: bool, position: EntityPosition, uuid: u128) -> Self {
        let generic = GenericEntity::new(
            air_remaining,
            distance_fallen,
            fire_ticks_left,
            is_invulnerable,
            velocity_vector,
            is_on_ground,
            position,
            uuid
        );

        match entity_type {
            EntityType::PLAYER=> {
                Entity::Player(
                    PlayerEntity {
                        base: generic,
                        inventory: Inventory{}
                    }
                )
            },
            EntityType::MOB => {
                Entity::Mob(
                    MobEntity {
                        base: generic,
                        id: "cubicle:null".to_string(),
                        extra: HashMap::new()
                    }
                )
            }
        }
    }

    fn base(&self) -> &GenericEntity {
        match self {
            Entity::Player(p) => &p.base,
            Entity::Mob(n) => &n.base
        }
    }

    fn base_mut(&mut self) -> &mut GenericEntity {
        match self {
            Entity::Player(p) => &mut p.base,
            Entity::Mob(n) => &mut n.base
        }
    }
}

impl GenericEntity {
    fn new(air_remaining: Tick, distance_fallen: f32, fire_ticks_left: Tick, is_invulnerable: bool, velocity_vector: (f32,f32,f32), is_on_ground: bool, position: EntityPosition, uuid: u128) -> Self {
        Self {
            air_remaining,
            distance_fallen,
            fire_ticks_left,
            is_invulnerable,
            uuid,
            is_on_ground,
            velocity_vector,
            position
        }
    }

    fn air_remaining(&self) -> &Tick { &self.air_remaining }
    fn distance_fallen(&self) -> &f32 { &self.distance_fallen }
    fn fire_ticks_left(&self) -> &Tick { &self.fire_ticks_left }
    fn is_invulnerable(&self) -> &bool { &self.is_invulnerable }
    fn velocity_vector(&self) -> &(f32, f32, f32) { &self.velocity_vector }
    fn is_on_ground(&self) -> &bool { &self.is_on_ground }
    fn position(&self) -> &EntityPosition { &self.position }

    fn set_air_remaining(&mut self, remaining: Tick) { self.air_remaining = remaining; }
    fn set_distance_fallen(&mut self, remaining: f32) { self.distance_fallen = remaining; }
    fn set_fire_ticks_left(&mut self, remaining: Tick) { self.fire_ticks_left = remaining; }
    fn set_is_vulnerable(&mut self, is_vulnerable: bool) {self.is_on_ground = is_vulnerable; }
    fn set_velocity_vector(&mut self, vector: (f32, f32, f32)) { self.velocity_vector = vector; }
    fn set_is_on_ground(&mut self, is_on_ground: bool) { self.is_on_ground = is_on_ground; }
    fn set_position(&mut self, position: EntityPosition) { self.position = position; }
}

