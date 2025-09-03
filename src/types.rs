use std::sync::{Arc, Mutex};
use crate::models::positions::chunk_position::ChunkPosition;
use crate::models::world::chunk::Chunk;
use crate::models::world::world::World;

pub type ChunkType<'a> = Arc<Mutex<Chunk<'a>>>;
pub type WorldType<'a> = Arc<Mutex<Box<World<'a>>>>;
pub type RegionPosition = ChunkPosition;