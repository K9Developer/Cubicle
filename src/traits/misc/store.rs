use crate::models::other::fast_set::FastSet;
use crate::models::other::position::Position;

pub trait StoreLike<T: Eq> {
    fn palette(&self) -> &FastSet<T>;
    fn add_item_to_palette(&mut self, item: T) -> usize; // return the index of the item in the palette
    fn set_item_index_at(&mut self, index: usize, palette_index: usize) -> bool;
    fn set_items_using_slice(&mut self, start_index: usize, palette_indices: &[usize]);
    fn get_palette_index_of_item(&self, item: &T) -> Option<usize>;
    fn set_item_at_index(&mut self, index: usize, item: T) -> bool;
    fn set_item_at_position(&mut self, relative_position: Position, item: T) -> bool;
    fn get_item_at_index(&self, index: usize) -> Option<T>;
    fn get_item_at_position(&self, relative_position: Position) -> Option<T>;
    fn indices_slice(&self) -> &[usize];
    fn indices_slice_mut(&mut self) -> &mut [usize];
}