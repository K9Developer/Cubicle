use crate::constants::versions::Version;
use crate::models::other::fast_set::FastSet;
use crate::models::other::position::Position;
use crate::models::world::block::Block;

#[derive(Debug)]
pub struct QuickLookupData {}

// NOTE: index 0 of palette is NULL BLOCK
#[derive(Debug)]
pub struct BlockStore<'a> {
    palette: FastSet<Block>,
    indices: Vec<usize>,
    qld: QuickLookupData,

    version: &'a Version,
}

// z -> x -> y
impl<'a> BlockStore<'a> {
    pub fn new(version: &Version) -> BlockStore {
        let height = version.data.lowest_y.abs() + version.data.highest_y.abs();
        let total_blocks = version.data.chunk_size * version.data.chunk_size * height;

        let mut p = FastSet::new();
        p.insert(Block::new_null());

        BlockStore {
            palette: p,
            indices: vec![0usize; total_blocks as usize],
            qld: QuickLookupData {},
            version,
        }
    }

    pub fn palette(&self) -> &FastSet<Block> { &self.palette }

    // returns new index
    #[inline(always)]
    pub fn add_block_to_palette(&mut self, block: Block) -> usize {
        self.palette.insert(block)
    }

    pub fn set_block_with_index(&mut self, index: usize, palette_index: usize) -> bool {
        if index > self.indices.len() - 1 {
            return false;
        };
        if palette_index >= self.palette.len() {
            return false;
        }
        self.indices[index] = palette_index;
        true
    }

    #[inline(always)]
    pub fn set_blocks_with_slice(&mut self, start_index: usize, mut palette_indices: &[usize]) {
        // Note: end_index is an extra one, this is because [a..b] is non inclusive (its extra because index starts with 0)
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

    fn get_index_of_block(&self, block: &Block) -> Option<usize> {
        for (i, b) in self.palette.iter().enumerate() {
            if !b.is_null() && b == block {
                return Some(i);
            }
        }
        None
    }

    pub fn set_block_at_index(&mut self, index: usize, block: Block) -> bool {
        if index > self.indices.len() - 1 {
            return false;
        };
        let palette_index = self
            .get_index_of_block(&block)
            .unwrap_or(self.palette.len());
        if palette_index == self.palette.len() {
            self.palette.insert(block);
        }
        self.indices[index] = palette_index;
        true
    }

    pub fn set_block_at_position(&mut self, relative_position: Position, block: Block) -> bool {
        let index = relative_position.to_index(self.version);
        self.set_block_at_index(index, block)
    }

    pub fn get_block_at_index(&self, index: usize) -> Option<Block> {
        if index > self.indices.len() - 1 {
            return None;
        }
        let palette_index = self.indices[index];
        let block = &self.palette[palette_index];
        if block.is_null() {
            None
        } else {
            Some((*block).clone())
        }
    }

    pub fn get_block_at_position(&self, relative_position: Position) -> Option<Block> {
        let index = relative_position.to_index(self.version);
        self.get_block_at_index(index)
    }

    pub fn indices_slice(&self) -> &[usize] {
        &self.indices
    }

    pub fn indices_slice_mut(&mut self) -> &mut [usize] {
        &mut self.indices
    }
}
