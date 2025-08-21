use std::collections::HashMap;
use fastnbt::Value;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NBTChunk {
    #[serde(rename = "DataVersion")]
    pub data_version: i32,
    #[serde(rename = "Entities")]
    pub entities: Vec<NBTEntity>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NBTEntity {
    #[serde(rename = "Air")]
    pub air_left: i16,
    #[serde(rename = "FallDistance")]
    pub distance_fallen: f32,
    #[serde(rename = "Fire")]
    pub fire_ticks_left: i16,
    #[serde(rename = "Invulnerable")]
    pub is_invulnerable: bool,
    #[serde(rename = "Motion")]
    pub motion: [f64; 3],
    #[serde(rename = "OnGround")]
    pub is_on_ground: bool,
    #[serde(rename = "Pos")]
    pub position: [f64; 3],
    #[serde(rename = "Rotation")]
    pub rotation: [f64; 2],
    #[serde(rename = "UUID")]
    pub uuid: Value,
    pub id: String,
    #[serde(flatten)]
    pub others: HashMap<String, Value>,
}