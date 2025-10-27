use crate::constants::constants::{BREWING_STAND_RESULT_COUNT, CAMPFIRE_ITEM_COUNT};
use crate::models::block_entity::block_entity::GenericBlockEntity;
use crate::models::other::inventory::{Inventory, Item};
use crate::models::other::tick::Tick;
use crate::traits::block_entity::BlockEntityTrait;
use crate::types::ItemSlot;

#[derive(Debug)]
pub struct CampfireBlockEntity {
    base: GenericBlockEntity,
    ticks_spent_cooking: Vec<Tick>,
    total_ticks_to_cook: Vec<Tick>,
    items: Inventory,
}

impl CampfireBlockEntity {
    pub fn new(
        base: GenericBlockEntity,
        ticks_spent_cooking: Vec<Tick>,
        total_ticks_to_cook: Vec<Tick>,
        items: Inventory,
    ) -> Self {
        CampfireBlockEntity {
            base, ticks_spent_cooking, total_ticks_to_cook, items
        }
    }

    pub fn items(&self) -> &Inventory { &self.items }
    pub fn ticks_spent_cooking_at(&self, slot: ItemSlot) -> Tick { self.ticks_spent_cooking[slot] }
    pub fn total_ticks_to_cook_at(&self, slot: ItemSlot) -> Tick { self.total_ticks_to_cook[slot] }
    pub fn ticks_left_to_cook_at(&self, slot: ItemSlot) -> Tick { self.total_ticks_to_cook[slot] - self.ticks_spent_cooking[slot] }
    pub fn ticks_spent_cooking_all(&self) -> &Vec<Tick> { &self.ticks_spent_cooking }
    pub fn total_ticks_to_cook_all(&self) -> &Vec<Tick> { &self.total_ticks_to_cook }
    pub fn ticks_left_to_cook_all(&self) -> Vec<Tick> {
        self.total_ticks_to_cook
            .iter()
            .zip(self.ticks_spent_cooking.iter())
            .map(|(total, spent)| *total - *spent)
            .collect()
    }
    pub fn cooking_items(&self) -> Vec<&Item> {
        (0..CAMPFIRE_ITEM_COUNT)
            .filter_map(|slot| self.items.get_at(slot))
            .collect()
    }
}

impl BlockEntityTrait for CampfireBlockEntity {
    fn base(&self) -> &GenericBlockEntity { &self.base }
    fn base_mut(&mut self) -> &mut GenericBlockEntity { &mut self.base }
}
