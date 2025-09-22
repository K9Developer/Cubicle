// biomes are kept as 4x4x4 cells in a chunk - BIOME_CELL_SIZE

use std::sync::Arc;
use crate::constants::constants::BIOME_CELL_SIZE;
use crate::constants::versions::Version;
use crate::models::other::fast_set::FastSet;
use crate::models::positions::whole_position::Position;
use crate::traits::misc::store::StoreLike;

#[derive(Debug)]
pub struct BiomeStore {
    palette: FastSet<String>,
    indices: Vec<usize>,

    version: Arc<Version>,
}

impl StoreLike<String> for BiomeStore {
    fn palette(&self) -> &FastSet<String> { &self.palette }
    fn add_item_to_palette(&mut self, biome: String) -> usize {
        self.palette.insert(biome)
    }
    fn set_item_index_at(&mut self, index: usize, palette_index: usize) -> bool {
        if index > self.indices.len() - 1 { return false; };
        if palette_index >= self.palette.len() { return false; }
        self.indices[index] = palette_index;
        true
    }
    #[inline(always)]
    fn set_items_using_slice(&mut self, start_index: usize, palette_indices: &[usize]) {
        // Note: end_index is an extra one, this is because [a..b] is non inclusive (its extra because index starts with 0)
        let end_index = start_index
            .checked_add(palette_indices.len())
            .expect("Too many biomes - Overflowed.");
        if end_index > self.indices.len() {
            panic!(
                "Too many biomes ({} biomes) for this version (max {} biomes)!",
                end_index,
                self.indices.len()
            );
        }
        self.indices[start_index..end_index].copy_from_slice(palette_indices);
    }
    fn get_palette_index_of_item(&self, biome: &String) -> Option<usize> {
        for (i, b) in self.palette.iter().enumerate() {
            if b != &"cubicle:null" && b == biome {
                return Some(i);
            }
        }
        None
    }
    fn set_item_at_index(&mut self, index: usize, biome: String) -> bool {
        if index > self.indices.len() - 1 { return false; };
        let palette_index = self.get_palette_index_of_biome(&biome).unwrap_or(self.palette.len());
        if palette_index == self.palette.len() { self.palette.insert(biome); }
        self.indices[index] = palette_index;
        true
    }
    fn set_item_at_position(&mut self, relative_position: Position, biome: String) -> bool {
        let index = relative_position.to_index(self.version.clone());
        self.set_biome_at_index(index, biome)
    }
    fn get_item_at_index(&self, index: usize) -> Option<String> {
        if index > self.indices.len() - 1 { return None; }
        let palette_index = self.indices[index];
        let biome = &self.palette[palette_index];
        if biome == &"cubicle:null" { None }
        else { Some(biome.clone()) }
    }
    fn get_item_at_position(&self, relative_position: Position) -> Option<String> {
        let index = relative_position.to_biome_index(self.version.clone());
        self.get_biome_at_index(index)
    }
    fn indices_slice(&self) -> &[usize] {
        &self.indices
    }
    fn indices_slice_mut(&mut self) -> &mut [usize] {
        &mut self.indices
    }
}

impl BiomeStore {
    pub fn new(version: Arc<Version>) -> Self {
        BiomeStore::with_palette_capacity(version, 2) // 2 is a random number - could be optimized
    }

    pub fn with_palette_capacity(version: Arc<Version>, size: usize) -> Self {
        let height = version.data.lowest_y.abs() + version.data.highest_y.abs();
        let total_biomes = (version.data.chunk_size * version.data.chunk_size * height) / BIOME_CELL_SIZE.pow(3);
        let mut p = FastSet::with_capacity(size);
        p.insert("cubicle:null".to_string());

        Self {
            version,
            palette: p,
            indices: vec![0usize; total_biomes as usize],
        }
    }

    #[inline]
    pub fn palette(&self) -> &FastSet<String> { <Self as StoreLike<String>>::palette(self) }
    #[inline(always)]
    pub fn add_biome_to_palette(&mut self, biome: String) -> usize { <Self as StoreLike<String>>::add_item_to_palette(self, biome) }
    pub fn set_biome_index_at(&mut self, index: usize, palette_index: usize) -> bool { <Self as StoreLike<String>>::set_item_index_at(self, index, palette_index) }
    #[inline(always)]
    pub fn set_biomes_using_slice(&mut self, start_index: usize, palette_indices: &[usize]) { <Self as StoreLike<String>>::set_items_using_slice(self, start_index, palette_indices) }
    pub fn get_palette_index_of_biome(&self, biome: &String) -> Option<usize> { <Self as StoreLike<String>>::get_palette_index_of_item(self, biome) }
    pub fn set_biome_at_index(&mut self, index: usize, biome: String) -> bool { <Self as StoreLike<String>>::set_item_at_index(self, index, biome) }
    pub fn set_biome_at_position(&mut self, relative_position: Position, biome: String) -> bool { <Self as StoreLike<String>>::set_item_at_position(self, relative_position, biome) }
    pub fn get_biome_at_index(&self, index: usize) -> Option<String> { <Self as StoreLike<String>>::get_item_at_index(self, index) }
    pub fn get_biome_at_position(&self, relative_position: Position) -> Option<String> { <Self as StoreLike<String>>::get_item_at_position(self, relative_position) }
    #[inline]
    pub fn indices_slice(&self) -> &[usize] { <Self as StoreLike<String>>::indices_slice(self) }
    #[inline]
    pub fn indices_slice_mut(&mut self) -> &mut [usize] { <Self as StoreLike<String>>::indices_slice_mut(self) }

    pub fn get_biome_at_block_position(&self, mut relative_position: Position) -> Option<String> {
        relative_position.set_x(relative_position.x() / BIOME_CELL_SIZE);
        relative_position.set_y(relative_position.y() / BIOME_CELL_SIZE);
        relative_position.set_z(relative_position.z() / BIOME_CELL_SIZE);
        self.get_biome_at_position(relative_position)
    }
}
