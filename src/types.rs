use std::sync::{Arc, Mutex};
use crate::models::positions::chunk_position::ChunkPosition;
use crate::models::world::chunk::Chunk;
use crate::models::world::world::World;

pub type ChunkType = Arc<Mutex<Chunk>>;
pub type WorldType<'a> = Arc<Mutex<Box<World<'a>>>>;
pub type RegionPosition = ChunkPosition;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum WorldKind {
    Singleplayer,
    Multiplayer,
}

pub enum HeightmapKind {
    MotionBlocking,
    MotionBlockingNoLeaves,
    Ground,
    SkyExposed
}
