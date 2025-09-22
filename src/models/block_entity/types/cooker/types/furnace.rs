use std::collections::HashMap;
use crate::models::block_entity::block_entity::GenericBlockEntity;
use crate::models::other::inventory::{Inventory, Item};
use crate::models::other::mojang_data::text_component::TextComponent;
use crate::models::other::tick::Tick;
use crate::traits::block_entity::LockableContainer;

pub struct FurnaceBlockEntity {
    base: GenericBlockEntity,
    items: Inventory,
    cook_history: HashMap<String, u32>,

    lock: Option<String>,
    custom_name: Option<TextComponent>,

    current_fuel_ticks_left: Tick,
    current_item_cooking_ticks_spent: Tick,
    total_cooking_ticks_for_current_item: Tick,
}

impl FurnaceBlockEntity {
    pub fn new(
        base: GenericBlockEntity,
        items: Inventory,
        cook_history: HashMap<String, u32>,
        lock: Option<String>,
        custom_name: Option<TextComponent>,
        current_fuel_ticks_left: Tick,
        current_item_cooking_ticks_spent: Tick,
        total_cooking_ticks_for_current_item: Tick,
    ) -> Self {
        FurnaceBlockEntity {
            base, items, cook_history, lock, custom_name, current_fuel_ticks_left, current_item_cooking_ticks_spent, total_cooking_ticks_for_current_item
        }
    }

    pub fn base(&self) -> &GenericBlockEntity { &self.base }
    pub fn items(&self) -> &Inventory { &self.items }
    pub fn cook_history(&self) -> &HashMap<String, u32> { &self.cook_history }
    pub fn current_fuel_ticks_left(&self) -> &Tick { &self.current_fuel_ticks_left }
    pub fn current_item_cooking_ticks_spent(&self) -> &Tick { &self.current_item_cooking_ticks_spent }
    pub fn total_cooking_ticks_for_current_item(&self) -> &Tick { &self.total_cooking_ticks_for_current_item }
    pub fn ticks_left_to_cook_for_current_item(&self) -> Tick {
        self.total_cooking_ticks_for_current_item - self.current_item_cooking_ticks_spent
    }
    pub fn fuel_item(&self) -> Option<&Item> { self.items.get_at(1) }
    pub fn cooking_item(&self) -> Option<&Item> { self.items.get_at(0) }
    pub fn result_item(&self) -> Option<&Item> { self.items.get_at(2) }
}

impl LockableContainer for FurnaceBlockEntity {
    fn is_locked(&self) -> bool { self.lock.is_some() }
    fn key(&self) -> &str { self.lock.as_deref().unwrap_or("") }
    fn set_key(&mut self, key: String) { self.lock = Some(key); }
    fn remove_lock(&mut self) { self.lock = None; }
}