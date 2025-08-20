use std::collections::HashMap;
use fastnbt::Value;
use serde::{Deserialize, Serialize};
use serde;

// TODO: use &'a str instead of String

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
    pub structures: NBTStructure,

    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NBTSection {
    #[serde(rename="Y")]
    pub y: i32,
    pub block_states: NBTBlockStates,
    pub biomes: NBTBiomeStates,

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
    pub palette: Vec<String>,
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
pub struct NBTStructure { // TODO: DO THIS
    #[serde(flatten)]
    pub others: HashMap<String, Value>,
}