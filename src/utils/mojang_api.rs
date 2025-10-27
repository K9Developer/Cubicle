use chrono::{DateTime, Utc};
use serde::Deserialize;
use reqwest::blocking::get;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::io;

use crate::constants::constants::MOJANG_VERSION_MANIFEST;

#[derive(Debug)]
pub enum MojangError {
    Http(reqwest::Error),
    Json(serde_json::Error),
    Io(io::Error),
    NotFound(String),
}

impl fmt::Display for MojangError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MojangError::Http(e) => write!(f, "HTTP error: {}", e),
            MojangError::Json(e) => write!(f, "JSON parse error: {}", e),
            MojangError::Io(e) => write!(f, "I/O error: {}", e),
            MojangError::NotFound(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for MojangError {}

impl From<reqwest::Error> for MojangError {
    fn from(e: reqwest::Error) -> Self {
        MojangError::Http(e)
    }
}

impl From<serde_json::Error> for MojangError {
    fn from(e: serde_json::Error) -> Self {
        MojangError::Json(e)
    }
}

impl From<io::Error> for MojangError {
    fn from(e: io::Error) -> Self {
        MojangError::Io(e)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VersionType {
    Snapshot,
    Release,
}

#[derive(Debug, Deserialize)]
pub struct LatestVersion {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Deserialize)]
pub struct ManifestVersion {
    pub id: String,

    #[serde(rename = "type")]
    pub version_type: VersionType,

    #[serde(rename = "url")]
    pub version_url: String,

    #[serde(rename = "releaseTime")]
    pub release_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct VersionManifest {
    pub latest: LatestVersion,
    pub versions: Vec<ManifestVersion>,
}

pub fn get_version_manifest() -> Result<VersionManifest, MojangError> {
    let api_data = get(MOJANG_VERSION_MANIFEST)?.text()?;
    Ok(serde_json::from_str(&api_data)?)
}

pub fn get_manifest_version(mut ver: &str) -> Result<ManifestVersion, MojangError> {
    let ver_man = get_version_manifest()?;
    let ver_id = if ver == "latest" || ver.is_empty() {
        ver_man.latest.release.clone()
    } else {
        ver.to_owned()
    };

    for opt_ver in ver_man.versions {
        if opt_ver.id == ver_id {
            return Ok(opt_ver);
        }
    }
    Err(MojangError::NotFound(format!("Version '{}' not found!", ver_id)))
}

pub fn get_latest_manifest_release() -> Result<ManifestVersion, MojangError> {
    get_manifest_version("latest")
}

pub fn get_server_jar_bytes(manifest_version: ManifestVersion) -> Result<Vec<u8>, MojangError> {
    let data = get(&manifest_version.version_url)?.text()?;
    let json: Value = serde_json::from_str(&data)?;
    let server_url = json["downloads"]["server"]["url"]
        .as_str()
        .ok_or_else(|| MojangError::NotFound("Server URL not found!".into()))?;
    let bytes = get(server_url)?.bytes()?;
    Ok(bytes.to_vec())
}
