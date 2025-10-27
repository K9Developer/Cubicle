use std::collections::HashMap;
use std::mem;
use fastnbt::Value;

pub mod extensions;
pub mod constants;
pub mod loaders;
pub mod models;
pub mod utils;
pub mod traits;
pub mod types;

trait ValueConversion {
    fn as_bool(&self) -> Option<bool>;
    fn as_i8(&self) -> Option<&i8>;
    fn as_i16(&self) -> Option<&i16>;
    fn as_i32(&self) -> Option<&i32>;
    fn as_i64(&self) -> Option<&i64>;
    fn as_f32(&self) -> Option<&f32>;
    fn as_f64(&self) -> Option<&f64>;
    fn as_string(&self) -> Option<&str>;
    fn as_i8_slice(&self) -> Option<&[i8]>;
    fn as_i32_slice(&self) -> Option<&[i32]>;
    fn as_value_slice(&self) -> Option<&Vec<Value>>;
    fn as_map(&self) -> Option<&HashMap<String, Value>>;
}

impl ValueConversion for Value {
    fn as_bool(&self) -> Option<bool> {
        match self { Value::Byte(b) => Some(*b != 0), _ => None }
    }

    fn as_i8(&self) -> Option<&i8> {
        match self { Value::Byte(b) => Some(b), _ => None }
    }

    fn as_i16(&self) -> Option<&i16> {
        match self { Value::Short(b) => Some(b), _ => None }
    }

    fn as_i32(&self) -> Option<&i32> {
        match self { Value::Int(b) => Some(b), _ => None }
    }

    fn as_i64(&self) -> Option<&i64> {
        match self { Value::Long(b) => Some(b), _ => None }
    }

    fn as_f32(&self) -> Option<&f32> {
        match self { Value::Float(b) => Some(b), _ => None }
    }

    fn as_f64(&self) -> Option<&f64> {
        match self { Value::Double(b) => Some(b), _ => None }
    }

    fn as_string(&self) -> Option<&str> {
        match self { Value::String(b) => Some(b), _ => None }
    }

    fn as_i8_slice(&self) -> Option<&[i8]> {
        match self { Value::ByteArray(b) => Some(b.iter().as_slice()), _ => None }
    }

    fn as_i32_slice(&self) -> Option<&[i32]> {
        match self { Value::IntArray(b) => Some(b.iter().as_slice()), _ => None }
    }

    fn as_value_slice(&self) -> Option<&Vec<Value>> {
        match self { Value::List(b) => Some(b), _ => None }
    }

    fn as_map(&self) -> Option<&HashMap<String, Value>> {
        match self { Value::Compound(b) => Some(b), _ => None }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I32(pub i32);

impl TryFrom<Value> for I32 {
    type Error = &'static str;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::Int(i) => Ok(I32(i)),
            Value::Short(i) => Ok(I32(i as i32)),
            Value::Byte(i) => Ok(I32(i as i32)),
            _ => Err("not an integer"),
        }
    }
}

unsafe fn transmute_map(m: HashMap<String, I32>) -> HashMap<String, i32> {
    // Safety: same key type and repr(transparent) guarantees identical layout - ChatGPT Code
    let ptr = &m as *const HashMap<String, I32> as *mut HashMap<String, i32>;
    let out = ptr.read();
    mem::forget(m);
    out
}