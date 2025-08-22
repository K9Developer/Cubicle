// TODO: Here put the main loader, it will handle lazy loading, dry loading, etc.

use crate::constants::versions::Version;
use crate::loaders::blocks_loader::loader::{get_block_loader, BlockLoader};
use crate::loaders::entities_loader::loader::{get_entity_loader, EntityLoader};

pub struct Loader<'a> {
    block_loader: Box<dyn BlockLoader<'a> + 'a>,
    entity_loader: Box<dyn EntityLoader<'a> + 'a>,
// player_loader
}

// TODO: Also make it possible to detect data version automatically - by that, version too maybe?
impl<'a> Loader<'a> {
    pub fn new(version: &'a Version) -> Self {
        Self {
            block_loader: get_block_loader(version),
            entity_loader: get_entity_loader(version)
        }
    }

    pub fn block_loader(&self) -> &Box<dyn BlockLoader<'a> + 'a> { &self.block_loader }
    pub fn entity_loader(&self) -> &Box<dyn EntityLoader<'a> + 'a> { &self.entity_loader }
}