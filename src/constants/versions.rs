use std::any::Any;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex, OnceLock};
use serde_json;
use super::config::config;
use serde::Deserialize;
use serde_json::{Value, Map, json};
use crate::types::WorldKind;
use crate::utils::mojang_api::{get_manifest_version, get_server_jar_bytes};

#[derive(Debug, Clone, Deserialize)]
pub struct DynamicVersionData {
    block_states: HashMap<String, String>
}

#[derive(Debug, Deserialize, Clone)]
pub struct VersionData {
    pub lowest_y: i32,
    pub highest_y: i32,
    pub chunk_size: i32,
    pub section_height: i32,
    pub version_data: i32,
    pub dynamic: DynamicVersionData
}

#[derive(Debug, Clone)]
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
    world_type: WorldKind,

    pub data: VersionData,
}

impl Version {
   pub fn new(version: &str, world_type: WorldKind) -> Version {
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

   pub const fn from_parts(major: u8, minor: u8, patch: u8, world_type: WorldKind, data: VersionData) -> Self {
       Self { major, minor, patch, world_type, data }
   }

   pub const fn major(&self) -> u8 { self.major }
   pub const fn minor(&self) -> u8 { self.minor }
   pub const fn patch(&self) -> u8 { self.patch }
    pub fn world_type(&self) -> &WorldKind { &self.world_type }
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

pub struct VersionManager {
    cache: Mutex<HashMap<(WorldKind, String), Arc<Version>>>,
}

static MANAGER: OnceLock<VersionManager> = OnceLock::new();

impl VersionManager {
    fn instance() -> &'static VersionManager { MANAGER.get_or_init(|| VersionManager { cache: Mutex::new(HashMap::new()), }) }

    pub fn get(id: &str, world_type: WorldKind) -> Arc<Version> {
        let this = Self::instance();
        let mut map = this.cache.lock().unwrap();
        let version_key = (world_type.clone(), id.to_string());
        if let Some(v) = map.get(&version_key) { return v.clone(); }
        let ver: Arc<Version> = Arc::new(Version::new(id, world_type));
        map.insert(version_key, ver.clone());
        ver
    }
}

fn get_dynamic_version_data(version: &str) -> Map<String, Value> {
    // let server = get_server_jar_bytes(get_manifest_version(version)?)?;
    // fetch online
    let mut h = Map::new();
    h.insert(String::from("block_states"), Value::Object(Map::new()));
    h
    // TODO: Fetch from cloud
}

fn get_version_data(version: &str) -> VersionData {
    let parsed: Value = serde_json::from_str(config::data_paths::GENERIC_VERSION_DATA).expect("JSON was not well-formatted");
    let obj = parsed.as_object().ok_or("top-level must be an object").unwrap();

    let generic = obj.get("generic").and_then(Value::as_object).ok_or("missing 'generic' object").unwrap();
    let per = obj.get(version).and_then(Value::as_object).ok_or("requested version not found").unwrap();

    let mut merged = Map::<String, Value>::new();
    for (k, v) in generic { merged.insert(k.clone(), v.clone()); }
    for (k, v) in per { merged.insert(k.clone(), v.clone()); }

    merged["dynamic"] = Value::Object(get_dynamic_version_data(version));

    serde_json::from_value(Value::Object(merged)).unwrap()
}
