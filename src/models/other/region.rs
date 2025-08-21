use std::path::PathBuf;
use crate::models::other::position::Position;

#[derive(Debug, Clone)]
pub enum RegionType {
    Block,
    Entity
}

#[derive(Debug)]
pub struct Region {
    pub position: Position,
    pub path: PathBuf,
    pub region_type: RegionType
}