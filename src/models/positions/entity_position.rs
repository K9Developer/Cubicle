use crate::models::other::lasso_string::LassoString;

#[derive(Debug, Clone)]
pub struct EntityPosition {
    x: f64,
    y: f64,
    z: f64,
    dimension: LassoString,
    rot_yaw: f64,
    rot_pitch: f64,
}


impl EntityPosition {
    pub fn new(x: f64, y: f64, z: f64, rot_yaw: f64, rot_pitch: f64, dimension: LassoString) -> Self {
        EntityPosition {
            x, y, z,
            dimension,
            rot_yaw,
            rot_pitch
        }
    }

    pub fn dimension(&self) -> &LassoString { &self.dimension }
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
