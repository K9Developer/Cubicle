use crate::models::block_entity::block_entity::GenericBlockEntity;
use crate::models::block_entity::types::storage_container::types::standard_container::StandardStorageContainerBlockEntity;
use crate::models::other::inventory::Inventory;
use crate::models::other::mojang_data::text_component::TextComponent;
use crate::traits::block_entity::{BlockEntityTrait, LockableContainer, StorageContainerTrait};

#[derive(Debug)]
pub struct HopperBlockEntity {
    base: StandardStorageContainerBlockEntity,
    transfer_cooldown: i32
}

impl HopperBlockEntity {
    pub fn new(base: StandardStorageContainerBlockEntity, transfer_cooldown: i32) -> Self {
        HopperBlockEntity {
            base, transfer_cooldown
        }
    }

    pub fn custom_name(&self) -> &Option<TextComponent> { &self.base.custom_name() }
    pub fn custom_name_mut(&mut self) -> Option<&mut TextComponent> { self.base.custom_name_mut() }
    pub fn transfer_cooldown(&self) -> i32 { self.transfer_cooldown }

    pub fn set_custom_name(&mut self, custom_name: Option<TextComponent>) { self.base.set_custom_name(custom_name); }
    pub fn set_transfer_cooldown(mut self, cooldown: i32){ self.transfer_cooldown = cooldown; }
}

impl BlockEntityTrait for HopperBlockEntity {
    fn base(&self) -> &GenericBlockEntity { &self.base.base() }
    fn base_mut(&mut self) -> &mut GenericBlockEntity { self.base.base_mut() }
}

impl StorageContainerTrait for HopperBlockEntity {
    fn items(&self) -> &Inventory { &self.base.items() }
    fn items_mut(&mut self) -> &mut Inventory { self.base.items_mut() }
    fn set_items(&mut self, items: Inventory) { self.base.set_items(items); }
}

impl LockableContainer for HopperBlockEntity {
    fn is_locked(&self) -> bool { self.base.is_locked() }
    fn key(&self) -> &str { self.base.key() }
    fn set_key(&mut self, key: String) { self.base.set_key(key) }
    fn remove_lock(&mut self) { self.base.remove_lock() }
}