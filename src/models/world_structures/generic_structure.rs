use fastnbt::Value;
use crate::models::other::position::Position;
use crate::models::other::properties::Properties;

pub struct BoundingBox {
    min_pos: Position,
    max_pos: Position,
}

pub struct GenericParentStructure {
    chunk_position: Position,
    id: String,
    children: Vec<GenericChildStructure>,
    extra: Properties
}

pub struct GenericChildStructure {
    pub(crate) bounding_box: BoundingBox,
    pub(crate) id: String,
    pub(crate) extra: Properties
}

impl GenericParentStructure {
    pub fn new(chunk_position: Position, id: &str, children: Vec<GenericChildStructure>, extra: Properties) -> GenericParentStructure {
        Self {
            chunk_position,
            id: id.to_string(),
            children,
            extra
        }
    }

    pub fn chunk_position(&self) -> &Position { &self.chunk_position }
    pub fn id(&self) -> &String { &self.id }
    pub fn children(&self) -> &Vec<GenericChildStructure> { &self.children }
    pub fn properties(&self) -> &Properties { &self.extra }
}

impl GenericChildStructure {
    pub fn new(id: &str, bounding_box: BoundingBox, extra: Properties) -> GenericChildStructure {
        Self {
            id: id.to_string(),
            bounding_box,
            extra
        }
    }

    pub fn bounding_box(&self) -> &BoundingBox { &self.bounding_box }
    pub fn properties(&self) -> &Properties { &self.extra }
}

impl BoundingBox {
    pub fn from_BB(bb_list: Value, dimension: &str) -> BoundingBox {
        if let Value::IntArray(bb_list) = bb_list {
            let bb_list: &[i32] = &*bb_list;
            BoundingBox {
                min_pos: Position::new(dimension, bb_list[0] as f32, bb_list[1] as f32, bb_list[2] as f32),
                max_pos: Position::new(dimension, bb_list[3] as f32, bb_list[4] as f32, bb_list[5] as f32),
            }
        } else {
            panic!("BoundingBox::from_bB called on non-array");
        }
    }
}