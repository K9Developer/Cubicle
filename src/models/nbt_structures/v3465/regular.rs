use std::collections::HashMap;
use fastnbt::Value;
use serde::{Deserialize, Serialize};
use serde;

// TODO: Remove as many properties as possible here so no parsing too much

#[derive(Serialize, Deserialize, Debug)]
pub struct NBTChunk {
    #[serde(rename="DataVersion")]
    pub data_version: i32,
    #[serde(rename="xPos")]
    pub x_pos: i32,
    #[serde(rename="yPos")]
    pub y_pos: i32,
    #[serde(rename="zPos")]
    pub z_pos: i32,
    #[serde(rename="LastUpdate")]
    pub last_update: i64,
    #[serde(rename="InhabitedTime")]
    pub inhabited_time: i64,
    #[serde(rename="Status")]
    pub status: String,

    pub sections: Vec<NBTSection>,
    pub block_entities: Vec<NBTBlockEntity>,
    pub fluid_ticks: Vec<NBTTileTick>,
    pub block_ticks: Vec<NBTTileTick>,
    pub structures: NBTStructureList,

    #[serde(rename="Heightmaps")]
    pub heightmaps: Heightmaps,

    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NBTSection {
    #[serde(rename="Y")]
    pub y: i32,
    pub block_states: Option<NBTBlockStates>,
    pub biomes: Option<NBTBiomeStates>,

    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NBTBlockStates {
    pub palette: Option<Vec<NBTBlockPalette>>,
    pub data: Option<Value>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NBTBlockPalette {
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="Properties")]
    pub properties: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NBTBiomeStates {
    pub palette: Option<Vec<String>>,
    pub data: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NBTBlockEntity {
    pub id: String,
    pub x: i32,
    pub y: i32,
    pub z: i32,

    #[serde(flatten)]
    pub others: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NBTTileTick {
    #[serde(rename="i")]
    pub id: String,
    #[serde(rename="p")]
    pub priority: i32,
    #[serde(rename="t")]
    pub time_until_tick: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NBTStructureList {
    #[serde(rename="References")]
    pub references: HashMap<String, Value>,
    pub starts: Option<HashMap<String, NBTStructure>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Heightmaps {
    #[serde(rename="MOTION_BLOCKING")]
    pub motion_blocking: Option<Value>, // highest block that blocks motion
    #[serde(rename="MOTION_BLOCKING_NO_LEAVES")]
    pub motion_blocking_no_leaves: Option<Value>, // highest block that blocks motion without leaves included
    #[serde(rename="OCEAN_FLOOR")]
    pub ocean_floor: Option<Value>, // the ground, no trees, motion blocking
    #[serde(rename="WORLD_SURFACE")]
    pub world_surface: Option<Value>, // the blocks that are exposed to sky (any)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NBTStructure {
    #[serde(rename="ChunkX")]
    pub chunk_x: i32,
    #[serde(rename="ChunkZ")]
    pub chunk_z: i32,
    #[serde(rename="id")]
    pub id: String,
    #[serde(rename="Children")]
    pub children: Option<Vec<NBTSubStructure>>,

    #[serde(flatten)]
    pub others: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NBTSubStructure {
    pub id: String,
    #[serde(rename="BB")]
    pub bounding_box: Value, // TODO: This should work as a [i32; 6] but it doesnt. Fix it and then go to the block loader and change the BoundingBox::from_BB impl and use

    #[serde(flatten)]
    pub others: HashMap<String, Value>
}