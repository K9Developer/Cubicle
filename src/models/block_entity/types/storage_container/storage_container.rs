use crate::models::block_entity::block_entity::GenericBlockEntity;
use crate::models::block_entity::types::storage_container::types::chiseled_bookshelf::ChiseledBookshelfBlockEntity;
use crate::models::block_entity::types::storage_container::types::hopper::HopperBlockEntity;
use crate::models::block_entity::types::storage_container::types::standard_container::StandardStorageContainerBlockEntity;
use crate::models::other::inventory::Inventory;
use crate::traits::block_entity::StorageContainerTrait;

// Standard - chest, trapped chest, barrel, shulker box, dispenser, dropper
// hopper
// chiseled bookshelf
pub enum StorageContainerBlockEntity {
    Chest(StandardStorageContainerBlockEntity),
    Barrel(StandardStorageContainerBlockEntity),
    ShulkerBox(StandardStorageContainerBlockEntity),
    Dispenser(StandardStorageContainerBlockEntity),
    Droper(StandardStorageContainerBlockEntity),

    Hopper(HopperBlockEntity),

    ChiseledBookshelf(ChiseledBookshelfBlockEntity),
}

impl StorageContainerTrait for StorageContainerBlockEntity {
    fn base(&self) -> &GenericBlockEntity {
        let this: &dyn StorageContainerTrait = self;
        this.base()
    }

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