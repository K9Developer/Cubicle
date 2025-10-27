use crate::models::block_entity::block_entity::GenericBlockEntity;
use crate::models::other::inventory::Inventory;
use crate::models::other::mojang_data::text_component::TextComponent;
use crate::traits::block_entity::{BlockEntityTrait, LockableContainer, StorageContainerTrait};


// TODO: loot table for chest - look in wiki
#[derive(Debug)]
pub struct StandardStorageContainerBlockEntity {
    base: GenericBlockEntity,
    lock: Option<String>,
    custom_name: Option<TextComponent>,
    items: Inventory,
}

impl StandardStorageContainerBlockEntity {
    pub fn new(base: GenericBlockEntity, lock: Option<String>, custom_name: Option<TextComponent>, items: Inventory) -> Self {
        StandardStorageContainerBlockEntity {
            base, lock, custom_name, items
        }
    }

    pub fn custom_name(&self) -> &Option<TextComponent> { &self.custom_name }
    pub fn custom_name_mut(&mut self) -> Option<&mut TextComponent> { self.custom_name.as_mut() }

    pub fn set_custom_name(&mut self, custom_name: Option<TextComponent>) { self.custom_name = custom_name; }
}

impl BlockEntityTrait for StandardStorageContainerBlockEntity {
    fn base(&self) -> &GenericBlockEntity { &self.base }
    fn base_mut(&mut self) -> &mut GenericBlockEntity {
        &mut self.base
    }
}

impl StorageContainerTrait for StandardStorageContainerBlockEntity {
    fn items(&self) -> &Inventory { &self.items }
    fn items_mut(&mut self) -> &mut Inventory { &mut self.items }
    fn set_items(&mut self, items: Inventory) { self.items = items; }
}

impl LockableContainer for StandardStorageContainerBlockEntity {
    fn is_locked(&self) -> bool { self.lock.is_some() }
    fn key(&self) -> &str { self.lock.as_deref().unwrap_or("") }
    fn set_key(&mut self, key: String) { self.lock = Some(key); }
    fn remove_lock(&mut self) { self.lock = None; }
}