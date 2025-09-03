use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::sync::Arc;
use crate::constants::constants::BIOME_CELL_SIZE;
use crate::constants::versions::Version;

fn div_floor(a: i32, b: i32) -> i32 {
    let div = a / b;
    let rem = a % b;
    if (rem != 0) && ((rem > 0) != (b > 0)) {
        div - 1
    } else {
        div
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    x: i32,
    y: i32,
    z: i32,
    dimension: String
}

#[derive(Debug, Clone)]
pub struct EntityPosition {
    x: f64,
    y: f64,
    z: f64,
    dimension: String,
    rot_yaw: f64,
    rot_pitch: f64,
}

impl Position {
    pub fn new(dimension: &str, x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z, dimension: dimension.into() }
    }

    pub fn from_index(index: i32, dimension: &str, chunk_position: Position, version: &Version) -> Position {
        let layer_size = version.data.chunk_size*version.data.chunk_size;
        let y = index / layer_size + version.data.lowest_y;
        let rest = index % layer_size;
        let x = chunk_position.x()*version.data.chunk_size + rest % version.data.chunk_size;
        let z = chunk_position.z()*version.data.chunk_size + rest / version.data.chunk_size;
        Position::new(dimension, x as i32, y as i32, z as i32)
    }

    pub fn dimension(&self) -> &str { &self.dimension }
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
    pub fn to_index(&self, version: Arc<Version>) -> usize {
        let chunk_size = version.data.chunk_size;
        if self.x() > chunk_size || self.z() > chunk_size { panic!("Chunk size out of range"); }
        let height = self.y() + version.data.lowest_y.abs();
        (height * chunk_size * chunk_size + self.x() * chunk_size + self.z()) as usize
    }

    #[inline]
    pub fn to_biome_index(&self, version: Arc<Version>) -> usize {
        let chunk_size = version.data.chunk_size;
        let chunk_biome_size = chunk_size / BIOME_CELL_SIZE;
        if self.x() > chunk_size || self.z() > chunk_size { panic!("Chunk size out of range"); }
        let height = (version.data.lowest_y.abs() / BIOME_CELL_SIZE) + self.y();
        (height * chunk_biome_size * chunk_biome_size + self.x() * chunk_biome_size + self.z()) as usize
    }

    pub fn to_block_coords(&self, chunk_size: i32) -> Position { // TODO: Add height maps so i can get current ground Y
        Position::new(self.dimension(), (self.x() * chunk_size) as i32, (self.y() * chunk_size) as i32, (self.z() * chunk_size) as i32)
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

impl EntityPosition {
    pub fn new(x: f64, y: f64, z: f64, rot_yaw: f64, rot_pitch: f64, dimension: &str) -> Self {
        EntityPosition {
            x, y, z,
            dimension: dimension.to_string(),
            rot_yaw,
            rot_pitch
        }
    }

    pub fn dimension(&self) -> &str { &self.dimension }
    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn z(&self) -> f64 { self.z }
    pub fn i_x(&self) -> i32 { self.x.floor() as i32 }
    pub fn i_y(&self) -> i32 { self.y.floor() as i32 }
    pub fn i_z(&self) -> i32 { self.z.floor() as i32 }
    pub fn yaw(&self) -> f64 { self.rot_yaw }
    pub fn pitch(&self) -> f64 { self.rot_pitch }

    pub fn set_x(&mut self, x: f64) { self.x = x }
    pub fn set_y(&mut self, y: f64) { self.y = y }
    pub fn set_z(&mut self, z: f64) { self.z = z }
    pub fn set_yaw(&mut self, rot_yaw: f64) { self.rot_yaw = rot_yaw; }
    pub fn set_pitch(&mut self, pitch: f64) { self.rot_pitch = pitch; }
    pub fn set_dimension(&mut self, dimension: &str) { self.dimension = dimension.into(); }
}

impl PartialEq for EntityPosition {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.dimension == other.dimension
            && self.rot_yaw == other.rot_yaw
            && self.rot_pitch == other.rot_pitch
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
        Position::new(t.3, t.0, t.1, t.2)
    }
}

