use crate::models::positions::whole_position::Position;

#[derive(Debug)]
pub enum TileTickType {
    FLUID,
    BLOCK
}

#[derive(Debug)]
pub struct TileTick {
    priority: i32,
    ticks_till_processed: i32,
    position: Position,
    _type: TileTickType,
}

impl TileTick {
    pub fn new(position: Position, priority: i32, ticks_till_processed: i32, _type: TileTickType) -> Self {
        Self {
            priority,
            ticks_till_processed,
            position,
            _type
        }
    }

    pub fn ticks_till_processed(&self) -> i32 { self.ticks_till_processed }
    pub fn priority(&self) -> i32 { self.priority }
    pub fn position(&self) -> &Position { &self.position }
    pub fn tile_tick_type(&self) -> &TileTickType { &self._type }

    pub fn set_priority(&mut self, priority: i32) { self.priority = priority; }
    pub fn set_position(&mut self, position: Position) { self.position = position; }
    pub fn set_ticks_till_processed(&mut self, ticks_till_processed: i32) { self.ticks_till_processed = ticks_till_processed; }
}