use crate::constants::versions::Version;
use crate::models::world::block::Block;
use crate::models::other::position::Position;

pub struct QuickLookupData {}

pub struct BlockStore<'a> {
    palette: Vec<Block>,
    indices: Vec<usize>,
    qld: QuickLookupData,

    version: &'a Version,
}

// z -> x -> y
impl BlockStore {
    pub fn new(version: &Version) -> BlockStore {
        let height = version.data.lowest_y.abs() + version.data.highest_y.abs();
        let total_blocks = version.data.chunk_size * version.data.chunk_size * height;

        BlockStore {
            palette: vec![Block::new_null()],
            indices: vec![0usize; total_blocks as usize],
            qld: QuickLookupData {},
            version
        }
    }

    pub fn set_palette(&mut self, palette: Vec<Block>) {
        self.palette = palette;
    }

    pub fn set_index(&mut self, index: usize, palette_index: usize) {
        self.indices[index] = palette_index;
    }

    fn get_index_of_block(&self, block: &Block) -> Option<usize> {
        for (i, b) in self.palette.iter().enumerate() {
            if !b.is_null() && b == block { return Some(i) }
        };
        None
    }

    pub fn set_block_at_index(&mut self, index: usize, block: Block) -> bool {
        if index > self.indices.len() - 1 { return false; };
        let palette_index = self.get_index_of_block(&block).unwrap_or(self.palette.len());
        if palette_index == self.palette.len() { self.palette.push(block); }
        self.indices[index] = palette_index;
        true
    }

    pub fn set_block_at_position(&mut self, relative_position: Position, block: Block) -> bool {
        let index = relative_position.to_index(self.version.data.chunk_size);
        self.set_block_at_index(index, block)
    }

    pub fn get_block_at_index(&self, index: usize) -> Option<Block> {
        if index > self.indices.len() - 1 { return None; }
        let palette_index = self.indices[index];
        let block = &self.palette[palette_index];
        if block.is_null() { None } else { Some((*block).clone()) }
    }

    pub fn get_block_at_position(&self, relative_position: Position) -> Option<Block> {
        let index = relative_position.to_index(self.version.data.chunk_size);
        self.get_block_at_index(index)
    }
}