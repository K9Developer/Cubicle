use std::collections::HashMap;
use crate::models::other::properties::Properties;
use crate::types::ItemSlot;

#[derive(Debug, Clone)]
pub struct Item {
    id: String,
    count: u32,
    properties: Properties
}

#[derive(Debug, Clone)]
pub struct Inventory {
    items: HashMap<ItemSlot, Item>, // this is saving space and allocation time but is slower
    size: usize
}

impl Inventory {
    pub fn new(size: usize) -> Self {
        Inventory { items: HashMap::new(), size }
    }

    pub fn get_all(&self) -> &HashMap<ItemSlot, Item> {
        &self.items
    }

    pub fn get_at(&self, slot: ItemSlot) -> Option<&Item> {
        self.items.get(&slot)
    }

    pub fn add_item(&mut self, item: Item) -> bool {
        for s in 0..self.size {
            if self.items.contains_key(&s) && self.items[&s].id == item.id  {
                let item = self.items.get_mut(&s);
                if let Some(item) = item { item.count += 1; }
                else { return false }
                return true;
            } else {
                self.items.insert(s, item);
                return true;
            }
        }
        false
    }

    pub fn add_item_at(&mut self, slot: ItemSlot, item: Item) -> bool {
        if slot >= self.size { return false; }
        if self.items.contains_key(&slot) && self.items[&slot].id == item.id {
            let item = self.items.get_mut(&slot);
            if let Some(item) = item { item.count += 1; }
            else { return false }
            true
        } else {
            self.items.insert(slot, item);
            true
        }
    }

    pub fn set_item(&mut self, slot: ItemSlot, item: Item) {
        self.items.insert(slot, item);
    }
}