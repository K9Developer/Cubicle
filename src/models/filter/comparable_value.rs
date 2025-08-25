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
    fn from_nbt_value(val: Value) -> Self {
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