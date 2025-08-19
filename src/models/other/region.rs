use std::path::PathBuf;
use crate::models::other::position::Position;

#[derive(Debug)]
pub struct Region {
    pub(crate) position: Position,
    pub(crate) path: PathBuf,
}