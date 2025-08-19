use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
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

#[derive(Debug)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
    dimension: String
}

impl Position {
    pub fn new(dimension: &str, x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, dimension: dimension.into() }
    }

    pub fn from_index(index: i32, dimension: &str, chunk_position: Position, version: &Version) -> Position {
        let layer_size = version.data.chunk_size*version.data.chunk_size;
        let y = index / layer_size + version.data.lowest_y;
        let rest = index % layer_size;
        let x = chunk_position.i_x()*version.data.chunk_size + rest % version.data.chunk_size;
        let z = chunk_position.i_z()*version.data.chunk_size + rest / version.data.chunk_size;
        Position::new(dimension, x as f32, y as f32, z as f32)
    }

    pub fn dimension(&self) -> &str { &self.dimension }
    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn z(&self) -> f32 { self.z }
    pub fn i_x(&self) -> i32 { self.x.floor() as i32 }
    pub fn i_y(&self) -> i32 { self.y.floor() as i32 }
    pub fn i_z(&self) -> i32 { self.z.floor() as i32 }

    pub fn set_x(&mut self, x: f32) { self.x = x }
    pub fn set_y(&mut self, y: f32) { self.y = y }
    pub fn set_z(&mut self, z: f32) { self.z = z }
    pub fn set_dimension(&mut self, dimension: &str) { self.dimension = dimension.into(); }

    pub fn distance(&self, other: &Self) -> Option<f32> {
        if self.dimension != other.dimension { return None; }
        Some(((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt())
    }

    pub fn to_block_coords(&self) -> (i32, i32, i32) {
        (self.x.floor() as i32, self.y.floor() as i32, self.z.floor() as i32)
    }

    #[inline]
    pub fn to_index(&self, version: &Version) -> usize {
        let chunk_size = version.data.chunk_size;
        if self.i_x() > chunk_size || self.i_z() > chunk_size { panic!("Chunk size out of range"); }
        let height = self.i_y() + version.data.lowest_y.abs();
        (height * chunk_size * chunk_size + self.i_x() * chunk_size + self.i_z()) as usize
    }

    #[inline]
    // Returns the (chunk_position, relative_block_pos_in_chunk)
    pub fn to_chunk_coords(&self, chunk_size: i32) -> ((i32, i32), Position) {
        let chunk_position = (self.i_x().div_euclid(chunk_size), self.i_z().div_euclid(chunk_size));

        (
            chunk_position,
            Position::new(&*self.dimension, self.i_x().rem_euclid(chunk_size) as f32, self.y(), self.i_x().rem_euclid(chunk_size) as f32)
        )
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        return other.x == self.x && other.y == self.y && other.z == self.z && self.dimension == other.dimension;
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:({}, {}, {})", self.dimension, self.x, self.y, self.z)
    }
}

impl Add<(f32, f32, f32)> for Position {
    type Output = Position;
    fn add(mut self, rhs: (f32, f32, f32)) -> Self::Output {
        self.x += rhs.0; self.y += rhs.1; self.z += rhs.2; self
    }
}
impl AddAssign<(f32, f32, f32)> for Position {
    fn add_assign(&mut self, rhs: (f32, f32, f32)) {
        self.x += rhs.0; self.y += rhs.1; self.z += rhs.2;
    }
}
impl Sub<(f32, f32, f32)> for Position {
    type Output = Position;
    fn sub(mut self, rhs: (f32, f32, f32)) -> Self::Output {
        self.x -= rhs.0; self.y -= rhs.1; self.z -= rhs.2; self
    }
}
impl SubAssign<(f32, f32, f32)> for Position {
    fn sub_assign(&mut self, rhs: (f32, f32, f32)) {
        self.x -= rhs.0; self.y -= rhs.1; self.z -= rhs.2;
    }
}

impl From<(f32, f32, f32, &str)> for Position {
    fn from(t: (f32, f32, f32, &str)) -> Self {
        Position::new(t.3, t.0, t.1, t.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div_floor() {
        assert_eq!(div_floor(7, 3), 2);
        assert_eq!(div_floor(6, 3), 2);
        assert_eq!(div_floor(1, 16), 0);

        assert_eq!(div_floor(-7, 3), -3);
        assert_eq!(div_floor(7, -3), -3);
        assert_eq!(div_floor(-1, 16), -1);

        assert_eq!(div_floor(-16, 16), -1);

        assert_eq!(div_floor(0, 16), 0);
    }

    #[test]
    fn test_int_accessors_and_block_coords() {
        let p = Position::new("overworld", 1.9, -0.1, -1.9);
        assert_eq!(p.i_x(), 1);
        assert_eq!(p.i_y(), -1);
        assert_eq!(p.i_z(), -2);
        assert_eq!(p.to_block_coords(), (1, -1, -2));
    }

    #[test]
    fn test_distance_same_dimension() {
        let a = Position::new("dim", 1.0, 2.0, 3.0);
        let b = Position::new("dim", 4.0, 6.0, 8.0);
        let d = a.distance(&b).unwrap();
        let expected = (((1.0 - 4.0) as f32).powi(2) + ((2.0 - 6.0) as f32).powi(2) + ((3.0 - 8.0) as f32).powi(2)).sqrt();
        assert!((d - expected).abs() < 1e-6);
        assert!((d - 7.0710678).abs() < 1e-6);
    }

    #[test]
    fn test_distance_different_dimension() {
        let a = Position::new("a", 0.0, 0.0, 0.0);
        let b = Position::new("b", 1.0, 1.0, 1.0);
        assert!(a.distance(&b).is_none());
    }

    #[test]
    fn test_add_sub_and_assign_ops() {
        let mut p = Position::new("d", 1.0, 2.0, 3.0);

        p += (0.5, -1.0, 2.5);
        assert!((p.x() - 1.5).abs() < 1e-6);
        assert!((p.y() - 1.0).abs() < 1e-6);
        assert!((p.z() - 5.5).abs() < 1e-6);

        p -= (0.5, 0.5, 0.5);
        assert!((p.x() - 1.0).abs() < 1e-6);
        assert!((p.y() - 0.5).abs() < 1e-6);
        assert!((p.z() - 5.0).abs() < 1e-6);

        let q = p + (1.0, 1.0, 1.0);
        assert!((q.x() - 2.0).abs() < 1e-6);
        assert!((q.y() - 1.5).abs() < 1e-6);
        assert!((q.z() - 6.0).abs() < 1e-6);

        let r = q - (2.0, 0.5, 1.0);
        assert!((r.x() - 0.0).abs() < 1e-6);
        assert!((r.y() - 1.0).abs() < 1e-6);
        assert!((r.z() - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_display_and_from_tuple() {
        let p: Position = (3.0, 4.0, 5.0, "nether").into();
        assert_eq!(p.dimension(), "nether");
        assert_eq!(format!("{}", p), "nether:(3, 4, 5)");
    }

    #[test]
    fn test_setters() {
        let mut p = Position::new("a", 0.0, 0.0, 0.0);
        p.set_dimension("b");
        p.set_x(1.25);
        p.set_y(-2.75);
        p.set_z(3.5);
        assert_eq!(p.dimension(), "b");
        assert!((p.x() - 1.25).abs() < 1e-6);
        assert!((p.y() + 2.75).abs() < 1e-6);
        assert!((p.z() - 3.5).abs() < 1e-6);
        assert_eq!(p.to_block_coords(), (1, -3, 3));
    }
}