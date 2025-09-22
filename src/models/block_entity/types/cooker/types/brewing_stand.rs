use crate::constants::constants::BREWING_STAND_RESULT_COUNT;
use crate::models::block_entity::block_entity::GenericBlockEntity;
use crate::models::other::inventory::{Inventory, Item};
use crate::models::other::mojang_data::text_component::TextComponent;
use crate::models::other::tick::Tick;
use crate::traits::block_entity::LockableContainer;

pub struct BrewingStandBlockEntity {
    base: GenericBlockEntity,
    items: Inventory,
    ticks_left_to_brew: Tick,
    applied_fuel_left: u8,
    lock: Option<String>,
    custom_name: Option<TextComponent>,
}

impl BrewingStandBlockEntity {
    fn new(
        base: GenericBlockEntity,
        items: Inventory,
        ticks_left_to_brew: Tick,
        applied_fuel_left: u8,
        lock: Option<String>,
        custom_name: Option<TextComponent>,
    ) -> Self {
        BrewingStandBlockEntity {
            base, items, ticks_left_to_brew, applied_fuel_left, lock, custom_name,
        }
    }

    pub fn base(&self) -> &GenericBlockEntity { &self.base }
    pub fn items(&self) -> &Inventory { &self.items }
    pub fn ticks_left_to_brew(&self) -> &Tick { &self.ticks_left_to_brew }
    pub fn applied_fuel_left(&self) -> u8 { self.applied_fuel_left } // 0-20
    pub fn custom_name(&self) -> Option<&TextComponent> { self.custom_name.as_ref() }
    pub fn fuel_item(&self) -> Option<&Item> { self.items.get_at(4) }
    pub fn ingredient_item(&self) -> Option<&Item> { self.items.get_at(3) }
    pub fn result_items(&self) -> Vec<&Item> {
        (0..BREWING_STAND_RESULT_COUNT)
            .filter_map(|slot| self.items.get_at(slot))
            .collect()
    }
}

impl LockableContainer for BrewingStandBlockEntity {
    fn is_locked(&self) -> bool { self.lock.is_some() }
    fn key(&self) -> &str { self.lock.as_deref().unwrap_or("") }
    fn set_key(&mut self, key: String) { self.lock = Some(key); }
    fn remove_lock(&mut self) { self.lock = None; }
}