use std::sync::{Arc, Mutex};
use crate::models::world::chunk::Chunk;
use crate::models::world::world::World;

pub type ChunkType<'a> = Arc<Mutex<Chunk<'a>>>;
pub type WorldType<'a> = Arc<Mutex<Box<World<'a>>>>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ChunkPosition {
    x: i32,
    z: i32,
    dimension: String
}

impl ChunkPosition {
    pub fn new(x: i32, z: i32, dimension: &str) -> ChunkPosition { ChunkPosition {x, z, dimension: dimension.to_string() } }
    pub fn x(&self) -> i32 { self.x }
    pub fn z(&self) -> i32 { self.z }
    pub fn dimension(&self) -> &str { &self.dimension }
    pub fn get_key(&self) -> ((i32, i32), String) { ((self.x, self.z), self.dimension.clone()) }
    pub fn position(&self) -> (i32, i32) {(self.x, self.z)}
    pub fn reference(&self) -> i64 {
        i64::from_be_bytes({
            let mut b = [0u8; 8];
            b[..4].copy_from_slice(&self.x().to_le_bytes()); // A2 FF FF FF
            b[4..].copy_from_slice(&self.z().to_le_bytes()); // 40 00 00 00
            b
        })
    }

    pub fn set_x(&mut self, x: i32) { self.x = x }
    pub fn set_z(&mut self, z: i32) { self.z = z }
    pub fn set_dimension(&mut self, d: String) { self.dimension = d; }
}