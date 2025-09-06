use crate::models::other::inventory::Inventory;
use crate::models::other::properties::Properties;
use crate::models::other::tick::Tick;
use crate::models::positions::entity_position::EntityPosition;

#[derive(Debug, Clone)]
pub struct GenericEntity {
    air_remaining: Tick,
    distance_fallen: f32,
    fire_ticks_left: Tick,
    is_invulnerable: bool,
    velocity_vector: (f64, f64, f64),
    is_on_ground: bool,
    position: EntityPosition,
    uuid: u128
}

pub enum EntityType {
    Player,
    Mob
}

#[derive(Debug, Clone)]
pub struct PlayerEntity { // TODO
    base: GenericEntity,
    inventory: Inventory,
    extra: Properties
}

#[derive(Debug, Clone)]
pub struct MobEntity {
    base: GenericEntity,
    id: String,
    extra: Properties
}

#[derive(Debug, Clone)]
pub enum Entity {
    Player(PlayerEntity),
    Mob(MobEntity),
}

impl Entity {
    pub fn new(entity_type: EntityType, air_remaining: Tick, distance_fallen: f32, fire_ticks_left: Tick, is_invulnerable: bool, velocity_vector: (f64,f64,f64), is_on_ground: bool, position: EntityPosition, uuid: u128, extra: Properties) -> Self {
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
            EntityType::Player => {
                Entity::Player(
                    PlayerEntity {
                        base: generic,
                        inventory: Inventory{},
                        extra
                    }
                )
            },
            EntityType::Mob => {
                Entity::Mob(
                    MobEntity {
                        base: generic,
                        id: "cubicle:null".to_string(),
                        extra
                    }
                )
            }
        }
    }

    pub fn base(&self) -> &GenericEntity {
        match self {
            Entity::Player(p) => &p.base,
            Entity::Mob(n) => &n.base
        }
    }

    pub fn base_mut(&mut self) -> &mut GenericEntity {
        match self {
            Entity::Player(p) => &mut p.base,
            Entity::Mob(n) => &mut n.base
        }
    }
}

impl GenericEntity {
    fn new(air_remaining: Tick, distance_fallen: f32, fire_ticks_left: Tick, is_invulnerable: bool, velocity_vector: (f64,f64,f64), is_on_ground: bool, position: EntityPosition, uuid: u128) -> Self {
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

    pub fn air_remaining(&self) -> &Tick { &self.air_remaining }
    pub fn distance_fallen(&self) -> &f32 { &self.distance_fallen }
    pub fn fire_ticks_left(&self) -> &Tick { &self.fire_ticks_left }
    pub fn is_invulnerable(&self) -> &bool { &self.is_invulnerable }
    pub fn velocity_vector(&self) -> &(f64, f64, f64) { &self.velocity_vector }
    pub fn is_on_ground(&self) -> &bool { &self.is_on_ground }
    pub fn position(&self) -> &EntityPosition { &self.position }

    pub fn set_air_remaining(&mut self, remaining: Tick) { self.air_remaining = remaining; }
    pub fn set_distance_fallen(&mut self, remaining: f32) { self.distance_fallen = remaining; }
    pub fn set_fire_ticks_left(&mut self, remaining: Tick) { self.fire_ticks_left = remaining; }
    pub fn set_is_vulnerable(&mut self, is_vulnerable: bool) {self.is_on_ground = is_vulnerable; }
    pub fn set_velocity_vector(&mut self, vector: (f64, f64, f64)) { self.velocity_vector = vector; }
    pub fn set_is_on_ground(&mut self, is_on_ground: bool) { self.is_on_ground = is_on_ground; }
    pub fn set_position(&mut self, position: EntityPosition) { self.position = position; }
}

impl MobEntity {
    pub fn new(id: String, air_remaining: Tick, distance_fallen: f32, fire_ticks_left: Tick, is_invulnerable: bool, velocity_vector: (f64,f64,f64), is_on_ground: bool, position: EntityPosition, uuid: u128, extra: Properties) -> Self {
        Self {
            base: GenericEntity::new(air_remaining, distance_fallen, fire_ticks_left, is_invulnerable, velocity_vector, is_on_ground, position, uuid),
            id,
            extra
        }
    }

    pub fn id(&self) -> &str { &self.id }
    pub fn properties(&self) -> &Properties { &self.extra }

    pub fn set_id(&mut self, id: String) { self.id = id; }
}

