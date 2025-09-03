use std::collections::HashMap;
use fastnbt::Value;
use crate::models::other::position::{EntityPosition, Position};

#[derive(Clone, Debug, PartialEq)]
pub enum ComparableValue { // TODO: Expand on this...
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    Position(Position),
    EntityPosition(EntityPosition),
    List(Vec<ComparableValue>),
    Map(HashMap<String, ComparableValue>),

    BoundingBox(Position, Position), // for within and positions only
}

impl ComparableValue {
    pub fn from_nbt_value(val: Value) -> Self {
        match val {
            Value::String(s) => ComparableValue::Text(s),
            Value::Byte(n) => ComparableValue::Int(n as i64),
            Value::Short(n) => ComparableValue::Int(n as i64),
            Value::Int(n) => ComparableValue::Int(n as i64),
            Value::Long(n) => ComparableValue::Int(n),
            Value::Float(n) => ComparableValue::Float(n as f64),
            Value::Double(n) => ComparableValue::Float(n),
            Value::IntArray(a) => ComparableValue::List(a.into_iter().map(|&x: &i32| ComparableValue::Int(x as i64)).collect()),
            Value::LongArray(a) => ComparableValue::List(a.into_iter().map(|&x: &i64| ComparableValue::Int(x)).collect()),
            Value::ByteArray(a) => ComparableValue::List(a.into_iter().map(|&x: &i8| ComparableValue::Int(x as i64)).collect()),
            Value::List(a) => ComparableValue::List(a.into_iter().map(ComparableValue::from_nbt_value).collect()),
            Value::Compound(m) => ComparableValue::Map(HashMap::from_iter(m.into_iter().map(|(k,v)| (k, ComparableValue::from_nbt_value(v))))),
        }
    }
}

impl PartialOrd for ComparableValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (ComparableValue::Int(x), ComparableValue::Int(y)) => x.partial_cmp(y),
            (ComparableValue::Float(x), ComparableValue::Float(y)) => x.partial_cmp(y),
            (ComparableValue::Text(x), ComparableValue::Text(y)) => x.partial_cmp(y),
            _ => None,
        }
    }
}

impl From<i32> for ComparableValue {
    fn from(val: i32) -> Self {
        ComparableValue::Int(val as i64)
    }
}

impl From<i64> for ComparableValue {
    fn from(val: i64) -> Self {
        ComparableValue::Int(val)
    }
}

impl From<f32> for ComparableValue {
    fn from(val: f32) -> Self {
        ComparableValue::Float(val as f64)
    }
}

impl From<f64> for ComparableValue {
    fn from(val: f64) -> Self {
        ComparableValue::Float(val)
    }
}

impl From<bool> for ComparableValue {
    fn from(val: bool) -> Self {
        ComparableValue::Bool(val)
    }
}

impl From<String> for ComparableValue {
    fn from(val: String) -> Self {
        ComparableValue::Text(val)
    }
}

impl From<&str> for ComparableValue {
    fn from(val: &str) -> Self {
        ComparableValue::Text(val.to_string())
    }
}

impl From<Position> for ComparableValue {
    fn from(val: Position) -> Self {
        ComparableValue::Position(val)
    }
}

impl From<EntityPosition> for ComparableValue {
    fn from(val: EntityPosition) -> Self {
        ComparableValue::EntityPosition(val)
    }
}

impl From<Vec<ComparableValue>> for ComparableValue {
    fn from(val: Vec<ComparableValue>) -> Self {
        ComparableValue::List(val)
    }
}

impl From<HashMap<String, ComparableValue>> for ComparableValue {
    fn from(val: HashMap<String, ComparableValue>) -> Self {
        ComparableValue::Map(val)
    }
}

impl From<(Position, Position)> for ComparableValue {
    fn from(val: (Position, Position)) -> Self {
        ComparableValue::BoundingBox(val.0, val.1)
    }
}