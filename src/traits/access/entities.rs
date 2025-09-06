use crate::models::entity::entity::Entity;
use crate::models::filter::filter::Filter;
use crate::models::world::fulls::full_entity::FullEntity;

pub trait EntityReader {
    fn entities<F>(&mut self, callback: F) where F: FnMut(FullEntity) -> bool;
    fn entity_count(&mut self) -> usize;
    fn find_entities<F>(&mut self, filter: Filter, callback: F) where F: FnMut(FullEntity) -> bool;
}

pub trait EntityWriter {
    fn set_entity_at_position(&mut self, entity: Entity) -> bool;
}