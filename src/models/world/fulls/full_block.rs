use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use fastnbt::Value;
use crate::models::block_entity::block_entity::BlockEntity;
use crate::models::other::properties::Properties;
use crate::models::positions::whole_position::Position;
use crate::models::world::block::PaletteBlock;
use crate::models::world::fulls::block_data::BlockData;
use crate::models::world::fulls::block_states::BlockStates;
use crate::types::{ChunkType, WorldType};
use crate::utils::position_utils::block_position_to_chunk_pos_and_block_index;

pub struct FullBlock<'a> {
    name: String,
    extra: BlockData,
    position: Position,
    null_flag: bool,

    world_ref: WorldType<'a>,
}

impl<'a> FullBlock<'a> {
    pub fn new(world_ref: &WorldType<'a>) -> Self {
        Self {
            name: "minecraft:air".to_string(),
            extra: BlockData::new(
                BlockStates::new(Properties::new(HashMap::new())),
                None
            ),
            null_flag: false,
            position: Position::new("overworld".into(), 0, 0, 0),
            world_ref: world_ref.clone(),
        }
    }

    pub fn new_with_data(world_ref: &WorldType<'a>, mut block: PaletteBlock, block_entity: Option<Rc<BlockEntity>>, position: Position) -> Self{
        Self {
            name: block.name().to_string(),
            extra: BlockData::new(
                BlockStates::new(block.properties().clone()),
                block_entity
            ),
            position,
            world_ref: world_ref.clone(),
            null_flag: false,
        }
    }

    pub fn id(&self) -> &str { &self.name }
    pub fn namespace(&self) -> &str { self.name.split(':').nth(0).unwrap_or("") }
    pub fn key(&self) -> &str {self.name.split(':').nth(1).unwrap_or("")}
    pub fn position(&self) -> &Position { &self.position }
    pub fn states(&self) -> Rc<RefCell<BlockStates>> { self.extra.states() }
    pub fn data(&self) -> Option<Rc<BlockEntity>> { self.extra.data() }
    pub fn palette_block(&self) -> PaletteBlock {
        PaletteBlock::new(&self.name, Some(self.extra.states().borrow().all().raw().clone()))
    }
    pub fn parent_chunk(&self) -> Option<(ChunkType, usize)> {
        let mut world = self.world_ref.lock().unwrap();
        let ver_data = world.version();
        let (chunk_pos, rel_index) = block_position_to_chunk_pos_and_block_index(&self.position, ver_data.data.chunk_size, ver_data.data.lowest_y);
        if let Some(dim) = world.dimension(chunk_pos.dimension()) {
            if let Some(ch) = dim.chunk(chunk_pos.position()) {
                return Some((ch, rel_index))
            }
            return None
        }
        None
    }


    pub fn set_id(&mut self, id: &str) { self.name = id.to_string(); }
    pub fn set_position(&mut self, position: Position) { self.position = position; }
    pub fn set_states(&mut self, states: BlockStates) { self.extra.set_states(states); }
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
            .field("extra", &self.extra.states().borrow().all())
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
            underlying: FullBlock::new(&world_ref),
        }
    }

    pub fn named(mut self, name: &str) -> Self { self.underlying.name = name.to_string(); self}
    pub fn at(mut self, pos: &Position) -> Self { self.underlying.position = pos.clone(); self }
    pub fn with_states(mut self, states: BlockStates) -> Self { self.underlying.extra.set_states(states); self}
    pub fn with_property(mut self, path: &str, value: Value) -> Self { self.underlying.extra.states().borrow_mut().all_mut().set(path, value); self }
    pub fn build(self) -> FullBlock<'a> {
        self.underlying
    }
}

