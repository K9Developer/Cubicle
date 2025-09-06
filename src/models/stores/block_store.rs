use std::sync::Arc;
use crate::constants::versions::Version;
use crate::models::other::fast_set::FastSet;
use crate::models::positions::whole_position::Position;
use crate::models::world::block::PaletteBlock;
use crate::traits::misc::store::StoreLike;

#[derive(Debug)]
pub struct QuickLookupData {}

// NOTE: index 0 of palette is NULL BLOCK

// z -> x -> y
#[derive(Debug)]
pub struct BlockStore {
    palette: FastSet<PaletteBlock>,
    indices: Vec<usize>,
    version: Arc<Version>,
    qld: QuickLookupData,
}

impl BlockStore {
    pub fn new(version: Arc<Version>) -> BlockStore {
        let height = (version.data.highest_y - version.data.lowest_y).abs() as i32;
        let total_blocks = (version.data.chunk_size * version.data.chunk_size * height) as usize;

        let mut p = FastSet::new();
        p.insert(PaletteBlock::new_null());

        BlockStore {
            palette: p,
            indices: vec![0usize; total_blocks],
            qld: QuickLookupData {},
            version,
        }
    }

    #[inline]
    pub fn palette(&self) -> &FastSet<PaletteBlock> { <Self as StoreLike<PaletteBlock>>::palette(self) }
    #[inline(always)]
    pub fn add_block_to_palette(&mut self, block: PaletteBlock) -> usize { <Self as StoreLike<PaletteBlock>>::add_item_to_palette(self, block) }
    pub fn set_block_with_index(&mut self, index: usize, palette_index: usize) -> bool { <Self as StoreLike<PaletteBlock>>::set_item_index_at(self, index, palette_index) }
    #[inline(always)]
    pub fn set_blocks_with_slice(&mut self, start_index: usize, palette_indices: &[usize]) { <Self as StoreLike<PaletteBlock>>::set_items_using_slice(self, start_index, palette_indices) }
    pub fn get_palette_index_of_block(&self, block: &PaletteBlock) -> Option<usize> { <Self as StoreLike<PaletteBlock>>::get_palette_index_of_item(self, block) }
    pub fn set_block_at_index(&mut self, index: usize, block: PaletteBlock) -> bool { <Self as StoreLike<PaletteBlock>>::set_item_at_index(self, index, block) }
    pub fn set_block_at_position(&mut self, relative_position: Position, block: PaletteBlock) -> bool { <Self as StoreLike<PaletteBlock>>::set_item_at_position(self, relative_position, block) }
    pub fn get_block_at_index(&self, index: usize) -> Option<PaletteBlock> { <Self as StoreLike<PaletteBlock>>::get_item_at_index(self, index) }
    pub fn get_block_at_position(&self, relative_position: Position) -> Option<PaletteBlock> { <Self as StoreLike<PaletteBlock>>::get_item_at_position(self, relative_position) }
    #[inline]
    pub fn indices_slice(&self) -> &[usize] { <Self as StoreLike<PaletteBlock>>::indices_slice(self) }
    #[inline]
    pub fn indices_slice_mut(&mut self) -> &mut [usize] { <Self as StoreLike<PaletteBlock>>::indices_slice_mut(self) }
    pub fn blocks(&self) -> impl Iterator<Item=PaletteBlock> {
        self.indices.iter().map(|i| self.palette[*i].clone())
    }
}

impl StoreLike<PaletteBlock> for BlockStore {
    #[inline]
    fn palette(&self) -> &FastSet<PaletteBlock> { &self.palette }

    #[inline(always)]
    fn add_item_to_palette(&mut self, block: PaletteBlock) -> usize {
        self.palette.insert(block)
    }

    fn set_item_index_at(&mut self, index: usize, palette_index: usize) -> bool {
        if index >= self.indices.len() { return false; }
        if palette_index >= self.palette.len() { return false; }
        self.indices[index] = palette_index;
        true
    }

    #[inline(always)]
    fn set_items_using_slice(&mut self, start_index: usize, palette_indices: &[usize]) {
        let end_index = start_index
            .checked_add(palette_indices.len())
            .expect("Too many blocks - Overflowed.");
        if end_index > self.indices.len() {
            panic!(
                "Too many blocks ({} blocks) for this version (max {} blocks)!",
                end_index,
                self.indices.len()
            );
        }
        self.indices[start_index..end_index].copy_from_slice(palette_indices);
    }

    fn get_palette_index_of_item(&self, block: &PaletteBlock) -> Option<usize> {
        for (i, b) in self.palette.iter().enumerate() {
            if !b.is_null() && b == block {
                return Some(i);
            }
        }
        None
    }

    fn set_item_at_index(&mut self, index: usize, block: PaletteBlock) -> bool {
        if index >= self.indices.len() { return false; }
        let palette_index = self
            .get_palette_index_of_item(&block)
            .unwrap_or(self.palette.len());
        if palette_index == self.palette.len() {
            self.palette.insert(block);
        }
        self.indices[index] = palette_index;
        true
    }

    fn set_item_at_position(&mut self, relative_position: Position, block: PaletteBlock) -> bool {
        let index = relative_position.to_index(self.version.clone());
        self.set_item_at_index(index, block)
    }

    fn get_item_at_index(&self, index: usize) -> Option<PaletteBlock> {
        if index >= self.indices.len() { return None; }
        let palette_index = self.indices[index];
        let block = &self.palette[palette_index];
        if block.is_null() { None } else { Some(block.clone()) }
    }

    fn get_item_at_position(&self, relative_position: Position) -> Option<PaletteBlock> {
        let index = relative_position.to_index(self.version.clone());
        self.get_item_at_index(index)
    }

    #[inline]
    fn indices_slice(&self) -> &[usize] { &self.indices }

    #[inline]
    fn indices_slice_mut(&mut self) -> &mut [usize] { &mut self.indices }
}
