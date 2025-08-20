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
    air_left: i16,
    #[serde(rename = "FallDistance")]
    distance_fallen: f32,
    #[serde(rename = "Fire")]
    fire_ticks_left: i16,
    #[serde(rename = "Invulnerable")]
    is_invulnerable: bool,
    #[serde(rename = "Motion")]
    motion: [f64; 3],
    #[serde(rename = "OnGround")]
    is_on_ground: bool,
    #[serde(rename = "Pos")]
    position: [f64; 3],
    #[serde(rename = "Rotation")]
    rotation: [f64; 2],
    #[serde(rename = "UUID")]
    uuid: Value,
}