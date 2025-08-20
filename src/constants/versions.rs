use std::cmp::Ordering;
use std::fmt;
use serde_json;
use super::config::config;
use serde::Deserialize;
use serde_json::{Value, Map};
use crate::models::world::world::WorldType;
// TODO: Need to not create a new version each time if they are the same.

#[derive(Debug, Deserialize, Clone)]
pub struct VersionData {
    pub lowest_y: i32,
    pub highest_y: i32,
    pub chunk_size: i32,
    pub section_height: i32,
    pub version_data: i32
}

#[derive(Debug, Clone)]
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
    world_type: WorldType,

    pub data: VersionData,
}

 impl Version {
    pub fn new(version: &str, world_type: WorldType) -> Version {
        let parts = version.split('.').collect::<Vec<&str>>();
        if parts.len() < 2 { panic!("Invalid version structure! expected xx.xx or xx.xx.xx") };
        Version {
            major: parts[0].parse::<u8>().unwrap(),
            minor: parts[1].parse::<u8>().unwrap(),
            patch: if parts.len() > 2 { parts[2].parse::<u8>().unwrap() } else { 0 },
            world_type,
            data: get_version_data(version)
        }
    }

    pub const fn from_parts(major: u8, minor: u8, patch: u8, world_type: WorldType, data: VersionData) -> Self {
        Self { major, minor, patch, world_type, data }
    }

    pub const fn major(&self) -> u8 { self.major }
    pub const fn minor(&self) -> u8 { self.minor }
    pub const fn patch(&self) -> u8 { self.patch }
     pub fn world_type(&self) -> &WorldType { &self.world_type }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        (self.major, self.minor, self.patch)
            == (other.major, other.minor, other.patch)
    }
}
impl Eq for Version {}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.major, self.minor, self.patch)
            .cmp(&(other.major, other.minor, other.patch))
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

fn get_version_data(version: &str) -> VersionData {
    let parsed: Value = serde_json::from_str(config::data_paths::VERSION_DATA).expect("JSON was not well-formatted");
    let obj = parsed.as_object().ok_or("top-level must be an object").unwrap();

    let generic = obj.get("generic").and_then(Value::as_object).ok_or("missing 'generic' object").unwrap();
    let per = obj.get(version).and_then(Value::as_object).ok_or("requested version not found").unwrap();

    let mut merged = Map::<String, Value>::new();
    for (k, v) in generic { merged.insert(k.clone(), v.clone()); }
    for (k, v) in per { merged.insert(k.clone(), v.clone()); }

    serde_json::from_value(Value::Object(merged)).unwrap()
}

/// --------- TESTS ---------

#[cfg(test)]
mod tests {
    use super::*;

    fn vd(lo: i32, hi: i32, cs: i32) -> VersionData {
        VersionData { lowest_y: lo, highest_y: hi, chunk_size: cs, section_height: 0, version_data: 0 }
    }

    #[test]
    fn display_and_accessors() {
        let v = Version::from_parts(1, 32, 3, WorldType::SINGLEPLAYER, vd(-64, 320, 16));
        assert_eq!(v.major(), 1);
        assert_eq!(v.minor(), 32);
        assert_eq!(v.patch(), 3);

        assert_eq!(format!("{}", v), "1.32.3");
    }

    #[test]
    fn ordering_by_tuple_semantics() {
        let a = Version::from_parts(1, 19, 4, WorldType::SINGLEPLAYER, vd(0, 0, 0));
        let b = Version::from_parts(1, 20, 0, WorldType::SINGLEPLAYER, vd(0, 0, 0));
        let c = Version::from_parts(1, 20, 1, WorldType::SINGLEPLAYER, vd(0, 0, 0));

        assert!(a < b);
        assert!(b < c);
        assert!(a < c);

        let a_same_nums = Version::from_parts(1, 19, 4, WorldType::SINGLEPLAYER, vd(-64, 320, 16));
        assert_eq!(a, a_same_nums);
    }

    #[test]
    fn equality_ignores_data_field() {
        let v1 = Version::from_parts(1, 18, 2, WorldType::SINGLEPLAYER, vd(-64, 256, 16));
        let v2 = Version::from_parts(1, 18, 2, WorldType::SINGLEPLAYER, vd(-128, 1024, 32));
        assert_eq!(v1, v2);

        let v3 = Version::from_parts(1, 18, 3, WorldType::SINGLEPLAYER, vd(-64, 256, 16));
        assert_ne!(v1, v3);
    }

    #[test]
    fn sorting_versions() {
        let mut v = vec![
            Version::from_parts(1, 20, 1, WorldType::SINGLEPLAYER, vd(0, 0, 0)),
            Version::from_parts(1, 19, 4, WorldType::SINGLEPLAYER, vd(0, 0, 0)),
            Version::from_parts(1, 20, 0, WorldType::SINGLEPLAYER, vd(0, 0, 0)),
            Version::from_parts(2, 0, 0, WorldType::SINGLEPLAYER, vd(0, 0, 0)),
        ];
        v.sort();

        let got: Vec<String> = v.into_iter().map(|x| format!("{}", x)).collect();
        assert_eq!(got, vec!["1.19.4", "1.20.0", "1.20.1", "2.0.0"]);
    }
}