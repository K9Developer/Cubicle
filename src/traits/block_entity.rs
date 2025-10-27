use crate::models::other::inventory::Inventory;
use crate::models::block_entity::prelude::*;

pub trait LockableContainer {
    fn is_locked(&self) -> bool;
    fn key(&self) -> &str;
    fn set_key(&mut self, key: String);
    fn remove_lock(&mut self);
}

pub trait StorageContainerTrait {
    fn items(&self) -> &Inventory;
    fn items_mut(&mut self) -> &mut Inventory;
    fn set_items(&mut self, items: Inventory);
}

pub trait BlockEntityTrait {
    fn base(&self) -> &GenericBlockEntity;
    fn base_mut(&mut self) -> &mut GenericBlockEntity;
}