use std::collections::HashMap;
use std::fmt;
use fastnbt::Value;
use crate::models::other::position::Position;
use crate::models::other::properties::Properties;
use crate::models::world::block::Block;
use crate::types::WorldType;
use crate::utils::position_utils::block_position_to_chunk_pos_and_block_index;

pub struct FullBlock<'a> {
    name: String,
    extra: Properties,
    position: Position,
    null_flag: bool,

    world_ref: WorldType<'a>,
}

impl<'a> FullBlock<'a> {
    pub fn new(world_ref: WorldType<'a>) -> Self {
        Self {
            name: "minecraft:air".to_string(),
            extra: Properties::new(HashMap::new()),
            null_flag: false,
            position: Position::new("minecraft:overworld", 0, 0, 0),
            world_ref: world_ref.clone(),
        }
    }

    pub fn new_with_data(world_ref: WorldType<'a>, block: Block, position: Position) -> Self{
        Self {
            name: block.name().to_string(),
            extra: block.properties().clone(), // TODO: Dont clone, we move the block here...
            position,
            world_ref,
            null_flag: false,
        }
    }

    pub fn id(&self) -> &str { &self.name }
    pub fn namespace(&self) -> &str { self.name.split(':').nth(1).unwrap_or("") }
    pub fn key(&self) -> &str {self.name.split(':').nth(2).unwrap_or("")}
    pub fn position(&self) -> &Position { &self.position }
    pub fn properties(&self) -> &Properties { &self.extra }
    pub fn palette_block(&self) -> Block {
        Block::new(&self.name, Some(self.extra.properties().clone()))
    }


    pub fn set_id(&mut self, id: &str) { self.name = id.to_string(); }
    pub fn set_position(&mut self, position: Position) { self.position = position; }
    pub fn set_properties(&mut self, properties: Properties) { self.extra = properties; }
    pub fn delete(&mut self) { self.name = "minecraft:air".to_string(); }

    pub fn commit(&self) -> bool {
        let mut world = self.world_ref.lock().unwrap();
        let ver_data = world.version();
        let (chunk_pos, block_index) = block_position_to_chunk_pos_and_block_index(self.position(), ver_data.data.chunk_size, ver_data.data.lowest_y);

        let dim = world.dimension_mut(self.position.dimension());
        if dim.is_none() { return false; }

        if let Some(chunk) = dim.unwrap().chunk_mut(chunk_pos.position()) {
            let mut locked_chunk = chunk.lock().unwrap();
            locked_chunk.block_store_mut().set_block_at_index(block_index, self.palette_block())
        } else { false }
    }

}

pub struct BlockBuilder<'a> {
    underlying: FullBlock<'a>,
}

impl<'a> fmt::Debug for FullBlock<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Block")
            .field("name", &self.name)
            .field("position", &self.position)
            .field("extra", &self.extra)
            .finish()
    }
}

impl<'a> fmt::Display for FullBlock<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FullBlock(name={}, at={})",
            self.name, self.position
        )
    }
}

impl<'a> BlockBuilder<'a> {
    pub fn new(world_ref: WorldType<'a>) -> Self {
        BlockBuilder {
            underlying: FullBlock::new(world_ref),
        }
    }

    pub fn named(mut self, name: &str) -> Self { self.underlying.name = name.to_string(); self}
    pub fn at(mut self, pos: &Position) -> Self { self.underlying.position = pos.clone(); self }
    pub fn with_properties(mut self, properties: Properties) -> Self { self.underlying.extra = properties; self}
    pub fn with_property(mut self, path: &str, value: Value) -> Self { self.underlying.extra.set(path, value); self }
    pub fn build(self) -> FullBlock<'a> {
        self.underlying
    }
}

