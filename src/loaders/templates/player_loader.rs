use std::path::PathBuf;
use std::sync::Arc;
use crate::constants::versions::Version;
use crate::loaders::v3465::player_loader::PlayerLoaderV3465;
use crate::models::entity::entity::PlayerEntity;

pub trait PlayerLoader<'a> {
    fn get_player_files(&self, world_path: PathBuf) -> Vec<PathBuf>;
    fn parse_player(&self, player_path: &PathBuf) -> PlayerEntity;
}

pub fn get_player_loader<'a>(version: Arc<Version>) -> Box<dyn PlayerLoader<'a>> {
    match version.data.version_data {
        3465 => Box::new( PlayerLoaderV3465 { version }),
        _ => panic!()
    }
}