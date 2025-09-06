use std::path::PathBuf;
use crate::types::RegionPosition;

#[derive(Debug, Clone)]
pub enum RegionType {
    Block,
    Entity
}

#[derive(Debug, Clone)]
pub struct Region {
    pub position: RegionPosition,
    pub path: PathBuf,
    pub region_type: RegionType
}