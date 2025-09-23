use crate::models::other::lasso_string::LassoString;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ChunkPosition {
    x: i32,
    z: i32,
    dimension: LassoString
}

impl ChunkPosition {
    pub fn new(x: i32, z: i32, dimension: LassoString) -> ChunkPosition { ChunkPosition {x, z, dimension } }
    pub fn x(&self) -> i32 { self.x }
    pub fn z(&self) -> i32 { self.z }
    pub fn dimension(&self) -> &LassoString { &self.dimension }
    pub fn get_key(&self) -> ((i32, i32), LassoString) { ((self.x, self.z), self.dimension) }
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
    pub fn set_dimension(&mut self, d: LassoString) { self.dimension = d; }
}