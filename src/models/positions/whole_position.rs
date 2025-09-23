use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::sync::Arc;
use crate::constants::constants::BIOME_CELL_SIZE;
use crate::constants::versions::Version;
use crate::models::other::lasso_string::LassoString;

#[derive(Debug, Clone)]
pub struct Position {
    x: i32,
    y: i32,
    z: i32,
    dimension: LassoString
}
impl Position {
    pub fn new(dimension: LassoString, x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z, dimension }
    }

    pub fn from_index(index: i32, dimension: LassoString, chunk_position: Position, version: &Version) -> Position {
        let layer_size = version.data.chunk_size*version.data.chunk_size;
        let y = index / layer_size + version.data.lowest_y;
        let rest = index % layer_size;
        let x = chunk_position.x()*version.data.chunk_size + rest % version.data.chunk_size;
        let z = chunk_position.z()*version.data.chunk_size + rest / version.data.chunk_size;
        Position::new(dimension, x as i32, y as i32, z as i32)
    }

    pub fn dimension(&self) -> &LassoString { &self.dimension }
    pub fn x(&self) -> i32 { self.x }
    pub fn y(&self) -> i32 { self.y }
    pub fn z(&self) -> i32 { self.z }

    pub fn set_x(&mut self, x: i32) { self.x = x }
    pub fn set_y(&mut self, y: i32) { self.y = y }
    pub fn set_z(&mut self, z: i32) { self.z = z }
    pub fn set_dimension(&mut self, dimension: &str) { self.dimension = dimension.into(); }

    pub fn distance(&self, other: &Self) -> Option<i32> {
        if self.dimension != other.dimension { return None; }
        Some(((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)).isqrt())
    }

    #[inline]
    pub fn to_index(&self, chunk_size: i32, lowest_y: i32) -> usize {
        if self.x() > chunk_size || self.z() > chunk_size { panic!("Chunk size out of range"); }
        let height = self.y() + lowest_y.abs();
        (height * chunk_size * chunk_size + self.x() * chunk_size + self.z()) as usize
    }

    #[inline]
    pub fn to_biome_index(&self, chunk_size: i32, lowest_y: i32) -> usize {
        let chunk_biome_size = chunk_size / BIOME_CELL_SIZE;
        if self.x() > chunk_size || self.z() > chunk_size { panic!("Chunk size out of range"); }
        let height = (lowest_y.abs() / BIOME_CELL_SIZE) + self.y() / BIOME_CELL_SIZE;
        (height * chunk_biome_size * chunk_biome_size + self.z() * chunk_biome_size + self.x()) as usize
    }

    pub fn to_block_coords(&self, chunk_size: i32) -> Position {
        Position::new(self.dimension().clone(), self.x() * chunk_size, self.y() * chunk_size, self.z() * chunk_size)
    }

    pub fn to_chunk_ref(&self) -> i64 {
        let x = self.x();
        let z = self.z();
        i64::from_be_bytes({
            let mut b = [0u8; 8];
            b[..4].copy_from_slice(&x.to_le_bytes()); // A2 FF FF FF
            b[4..].copy_from_slice(&z.to_le_bytes()); // 40 00 00 00
            b
        })
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        other.x == self.x && other.y == self.y && other.z == self.z && self.dimension == other.dimension
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:({}, {}, {})", self.dimension, self.x, self.y, self.z)
    }
}

impl Add<(i32, i32, i32)> for Position {
    type Output = Position;
    fn add(mut self, rhs: (i32, i32, i32)) -> Self::Output {
        self.x += rhs.0; self.y += rhs.1; self.z += rhs.2; self
    }
}
impl AddAssign<(i32, i32, i32)> for Position {
    fn add_assign(&mut self, rhs: (i32, i32, i32)) {
        self.x += rhs.0; self.y += rhs.1; self.z += rhs.2;
    }
}
impl Sub<(i32, i32, i32)> for Position {
    type Output = Position;
    fn sub(mut self, rhs: (i32, i32, i32)) -> Self::Output {
        self.x -= rhs.0; self.y -= rhs.1; self.z -= rhs.2; self
    }
}
impl SubAssign<(i32, i32, i32)> for Position {
    fn sub_assign(&mut self, rhs: (i32, i32, i32)) {
        self.x -= rhs.0; self.y -= rhs.1; self.z -= rhs.2;
    }
}

impl From<(i32, i32, i32, &str)> for Position {
    fn from(t: (i32, i32, i32, &str)) -> Self {
        Position::new(t.3.into(), t.0, t.1, t.2)
    }
}

impl From<(i32, i32, i32, LassoString)> for Position {
    fn from(t: (i32, i32, i32, LassoString)) -> Self {
        Position::new(t.3, t.0, t.1, t.2)
    }
}