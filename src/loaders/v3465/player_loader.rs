use std::path::PathBuf;
use std::sync::Arc;
use crate::constants::versions::Version;
use crate::loaders::templates::player_loader;
use crate::loaders::templates::player_loader::PlayerLoader;
use crate::loaders::v3465::utils::entity_utils::parse_nbt_entity;
use crate::models::entity::entity::PlayerEntity;
use crate::models::nbt_structures::v3465::entities::NBTChunk;
use crate::models::other::lasso_string::LassoString;

pub struct PlayerLoaderV3465 {
    pub version: Arc<Version>
}

impl<'a> PlayerLoader<'a> for PlayerLoaderV3465 {
    fn get_player_files(&self, world_path: PathBuf) -> Vec<PathBuf> {
        todo!()
    }

    fn parse_player(&self, player_path: &PathBuf) -> PlayerEntity {
        todo!()
    }
}