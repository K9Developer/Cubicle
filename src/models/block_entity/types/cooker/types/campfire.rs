use crate::constants::constants::{BREWING_STAND_RESULT_COUNT, CAMPFIRE_ITEM_COUNT};
use crate::models::block_entity::block_entity::GenericBlockEntity;
use crate::models::other::inventory::{Inventory, Item};
use crate::models::other::tick::Tick;
use crate::types::ItemSlot;

pub struct CampfireBlockEntity {
    base: GenericBlockEntity,
    ticks_spent_cooking: [Tick; CAMPFIRE_ITEM_COUNT],
    total_ticks_to_cook: [Tick; CAMPFIRE_ITEM_COUNT],
    items: Inventory,
}

impl CampfireBlockEntity {
    pub fn new(
        base: GenericBlockEntity,
        ticks_spent_cooking: [Tick; CAMPFIRE_ITEM_COUNT],
        total_ticks_to_cook: [Tick; CAMPFIRE_ITEM_COUNT],
        items: Inventory,
    ) -> Self {
        CampfireBlockEntity {
            base, ticks_spent_cooking, total_ticks_to_cook, items
        }
    }

    pub fn base(&self) -> &GenericBlockEntity { &self.base }
    pub fn items(&self) -> &Inventory { &self.items }
    pub fn ticks_spent_cooking_at(&self, slot: ItemSlot) -> Tick { self.ticks_spent_cooking[slot] }
    pub fn total_ticks_to_cook_at(&self, slot: ItemSlot) -> Tick { self.total_ticks_to_cook[slot] }
    pub fn ticks_left_to_cook_at(&self, slot: ItemSlot) -> Tick { self.total_ticks_to_cook[slot] - self.ticks_spent_cooking[slot] }
    pub fn ticks_spent_cooking_all(&self) -> &[Tick; CAMPFIRE_ITEM_COUNT] { &self.ticks_spent_cooking }
    pub fn total_ticks_to_cook_all(&self) -> &[Tick; CAMPFIRE_ITEM_COUNT] { &self.total_ticks_to_cook }
    pub fn ticks_left_to_cook_all(&self) -> [Tick; CAMPFIRE_ITEM_COUNT] {
        std::array::from_fn(|i| self.total_ticks_to_cook[i] - self.ticks_spent_cooking[i])
    }
    pub fn cooking_items(&self) -> Vec<&Item> {
        (0..CAMPFIRE_ITEM_COUNT)
            .filter_map(|slot| self.items.get_at(slot))
            .collect()
    }
}