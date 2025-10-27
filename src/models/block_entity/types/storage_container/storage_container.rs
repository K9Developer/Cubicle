use crate::models::block_entity::block_entity::GenericBlockEntity;
use crate::models::block_entity::types::storage_container::types::chiseled_bookshelf::ChiseledBookshelfBlockEntity;
use crate::models::block_entity::types::storage_container::types::hopper::HopperBlockEntity;
use crate::models::block_entity::types::storage_container::types::standard_container::StandardStorageContainerBlockEntity;
use crate::models::other::inventory::Inventory;
use crate::traits::block_entity::{BlockEntityTrait, StorageContainerTrait};

// Normal - chest, trapped chest, barrel, shulker box,
// Spitter - dispenser, dropper
// hopper
// chiseled bookshelf
#[derive(Debug)]
pub enum StorageContainerBlockEntity {
    Normal(StandardStorageContainerBlockEntity),
    Spitter(StandardStorageContainerBlockEntity),

    Hopper(HopperBlockEntity),

    ChiseledBookshelf(ChiseledBookshelfBlockEntity),
}

impl BlockEntityTrait for StorageContainerBlockEntity {
    fn base(&self) -> &GenericBlockEntity {
        match self {
            StorageContainerBlockEntity::Normal(a) => a.base(),
            StorageContainerBlockEntity::Spitter(a) => a.base(),
            StorageContainerBlockEntity::Hopper(a) => a.base(),
            StorageContainerBlockEntity::ChiseledBookshelf(a) => a.base(),
        }
    }

    fn base_mut(&mut self) -> &mut GenericBlockEntity {
        match self {
            StorageContainerBlockEntity::Normal(a) => a.base_mut(),
            StorageContainerBlockEntity::Spitter(a) => a.base_mut(),
            StorageContainerBlockEntity::Hopper(a) => a.base_mut(),
            StorageContainerBlockEntity::ChiseledBookshelf(a) => a.base_mut(),
        }
    }
}

impl StorageContainerTrait for StorageContainerBlockEntity {

    fn items(&self) -> &Inventory {
        let this: &dyn StorageContainerTrait = self;
        this.items()
    }

    fn items_mut(&mut self) -> &mut Inventory {
        let this: &mut dyn StorageContainerTrait = self;
        this.items_mut()
    }

    fn set_items(&mut self, items: Inventory) {
        let this: &mut dyn StorageContainerTrait = self;
        this.set_items(items)
    }
}

impl StorageContainerBlockEntity {
    pub fn as_normal(&self) -> Option<&StandardStorageContainerBlockEntity> {
        match self {
            StorageContainerBlockEntity::Normal(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_spitter(&self) -> Option<&StandardStorageContainerBlockEntity> {
        match self {
            StorageContainerBlockEntity::Normal(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_hopper(&self) -> Option<&HopperBlockEntity> {
        match self {
            StorageContainerBlockEntity::Hopper(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_chiseled_bookshelf(&self) -> Option<&ChiseledBookshelfBlockEntity> {
        match self {
            StorageContainerBlockEntity::ChiseledBookshelf(a) => Some(a),
            _ => None,
        }
    }
}