use crate::models::block_entity::block_entity::GenericBlockEntity;
use crate::models::block_entity::types::cooker::types::brewing_stand::BrewingStandBlockEntity;
use crate::models::block_entity::types::cooker::types::campfire::CampfireBlockEntity;
use crate::models::block_entity::types::cooker::types::furnace::FurnaceBlockEntity;
use crate::models::other::inventory::Item;
use crate::traits::block_entity::BlockEntityTrait;

// Furnace - furnace, smoker, blast furnace
// campfire - campfire, soul campfire,
// brewing stand
#[derive(Debug)]
pub enum CookerBlockEntity {
    Furnace(FurnaceBlockEntity),
    Campfire(CampfireBlockEntity),
    BrewingStand(BrewingStandBlockEntity),
}

impl BlockEntityTrait for CookerBlockEntity {
    fn base(&self) -> &GenericBlockEntity {
        match self {
            CookerBlockEntity::Furnace(a) => a.base(),
            CookerBlockEntity::Campfire(a) => a.base(),
            CookerBlockEntity::BrewingStand(a) => a.base(),
        }
    }
    fn base_mut(&mut self) -> &mut GenericBlockEntity {
        match self {
            CookerBlockEntity::Furnace(a) => a.base_mut(),
            CookerBlockEntity::Campfire(a) => a.base_mut(),
            CookerBlockEntity::BrewingStand(a) => a.base_mut(),
        }
    }
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

impl CookerBlockEntity {
    pub fn as_furnace(&self) -> Option<&FurnaceBlockEntity> {
        match self {
            CookerBlockEntity::Furnace(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_campfire(&self) -> Option<&CampfireBlockEntity> {
        match self {
            CookerBlockEntity::Campfire(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_brewing_stand(&self) -> Option<&BrewingStandBlockEntity> {
        match self {
            CookerBlockEntity::BrewingStand(a) => Some(a),
            _ => None,
        }
    }
}