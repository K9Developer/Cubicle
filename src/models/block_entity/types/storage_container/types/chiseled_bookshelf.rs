use crate::models::block_entity::block_entity::GenericBlockEntity;
use crate::models::block_entity::types::storage_container::types::hopper::HopperBlockEntity;
use crate::models::block_entity::types::storage_container::types::standard_container::StandardStorageContainerBlockEntity;
use crate::models::other::inventory::Inventory;
use crate::models::other::mojang_data::text_component::TextComponent;
use crate::traits::block_entity::{LockableContainer, StorageContainerTrait};

pub struct ChiseledBookshelfBlockEntity {
    base: StandardStorageContainerBlockEntity, // this doesnt have a custom name so its a waste but better DX
    last_interacted_slot: i32
}

impl ChiseledBookshelfBlockEntity {
    pub fn new(base: StandardStorageContainerBlockEntity, last_interacted_slot: i32) -> Self {
        ChiseledBookshelfBlockEntity {
            base, last_interacted_slot
        }
    }

    pub fn last_interacted_slot(&self) -> i32 { self.last_interacted_slot }

    pub fn set_last_interacted_slot(mut self, last_interacted_slot: i32){ self.last_interacted_slot = last_interacted_slot; }
}

impl StorageContainerTrait for ChiseledBookshelfBlockEntity {
    fn base(&self) -> &GenericBlockEntity { &self.base.base() }
    fn items(&self) -> &Inventory { &self.base.items() }
    fn items_mut(&mut self) -> &mut Inventory { self.base.items_mut() }
    fn set_items(&mut self, items: Inventory) { self.base.set_items(items); }
}

impl LockableContainer for ChiseledBookshelfBlockEntity {
    fn is_locked(&self) -> bool { self.base.is_locked() }
    fn key(&self) -> &str { self.base.key() }
    fn set_key(&mut self, key: String) { self.base.set_key(key) }
    fn remove_lock(&mut self) { self.base.remove_lock() }
}