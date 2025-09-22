use crate::models::block_entity::types::cooker::types::brewing_stand::BrewingStandBlockEntity;
use crate::models::block_entity::types::cooker::types::campfire::CampfireBlockEntity;
use crate::models::block_entity::types::cooker::types::furnace::FurnaceBlockEntity;
use crate::models::other::inventory::Item;

// furnace, smoker, blast furnace, campfire, soul campfire, brewing stand
pub enum CookerBlockEntity {
    Furnace(FurnaceBlockEntity),
    Campfire(CampfireBlockEntity),
    BrewingStand(BrewingStandBlockEntity),
}

impl CookerBlockEntity {
    pub fn get_fuel_item(&self) -> Option<&Item> {
        match self {
            CookerBlockEntity::Furnace(e) => e.fuel_item(),
            CookerBlockEntity::BrewingStand(e) => e.fuel_item(),
            _ => None,
        }
    }

    pub fn get_item_results(&self) -> Option<Vec<&Item>> {
        match self {
            CookerBlockEntity::Furnace(e) => {
                if let Some(r) = e.result_item() { Some(vec![r]) }
                else { None }
            },
            CookerBlockEntity::BrewingStand(e) => Some(e.result_items()),
            _ => None,
        }
    }

    pub fn get_cooking_items(&self) -> Option<Vec<&Item>> {
        match self {
            CookerBlockEntity::Furnace(e) => {
                if let Some(r) = e.cooking_item() { Some(vec![r]) }
                else { None }
            },
            CookerBlockEntity::BrewingStand(e) => {
                if let Some(r) = e.ingredient_item() { Some(vec![r]) }
                else { None }
            },
            CookerBlockEntity::Campfire(e) => Some(e.cooking_items()),
        }
    }
}
