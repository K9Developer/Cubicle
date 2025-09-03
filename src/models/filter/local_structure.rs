use std::collections::HashMap;
use crate::models::world::block::Block;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Offset {
    pub dx: i16,
    pub dy: i16,
    pub dz: i16,
}

impl From<(i32, i32, i32)> for Offset {
    fn from(t: (i32, i32, i32)) -> Self {
        Self { dx: t.0 as i16, dy: t.1 as i16, dz: t.2 as i16 }
    }
}
impl From<(i16, i16, i16)> for Offset {
    fn from(t: (i16, i16, i16)) -> Self {
        Self { dx: t.0, dy: t.1, dz: t.2 }
    }
}


#[derive(Clone, Debug)]
pub struct LocalStructure {
    block_map: HashMap<Offset, Block>,
}

impl LocalStructure {
    pub fn new() -> Self {
        Self { block_map: HashMap::new() }
    }

    pub fn add<O, B>(mut self, offset: O, block: B) -> Self
    where
        O: Into<Offset>,
        B: Into<Block>,
    {
        self.block_map.insert(offset.into(), block.into());
        self
    }

    pub fn iter(&self) -> impl Iterator<Item = (Offset, &Block)> { self.block_map.iter().map(|(k, v)| (*k, v)) }
    pub fn len(&self) -> usize { self.block_map.len() }
    pub fn is_empty(&self) -> bool { self.block_map.is_empty() }
}