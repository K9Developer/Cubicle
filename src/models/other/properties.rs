use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use fastnbt::Value;

#[derive(Clone, Debug)]
pub struct Properties {
    props: Arc<Mutex<HashMap<String, Value>>>,
}

impl Properties {
    pub fn new(props: HashMap<String, Value>) -> Properties {
        Properties { props: Arc::from(Mutex::new(props)) }
    }

    pub fn raw(&self) -> MutexGuard<HashMap<String, Value>> {
        self.props.lock().unwrap()
    }

    pub fn properties_raw(&self) -> Arc<Mutex<HashMap<String, Value>>> {
        self.props.clone()
    }

    pub fn get(&self, path: &str) -> Option<Value> {
        let mut parts = path.split('.');
        let props = self.raw();
        let mut current_node = props.get(parts.next()?)?;

        for part in parts {
            match current_node {
                Value::List(a) => {
                    let idx: usize = part.parse().ok()?;
                    current_node = a.get(idx)?;
                }
                Value::Compound(m) => {
                    current_node = m.get(part)?;
                }
                Value::ByteArray(a) => {
                    let idx: usize = part.parse().ok()?;
                    return Some(Value::Byte(*a.get(idx)?));
                }
                Value::IntArray(a) => {
                    let idx: usize = part.parse().ok()?;
                    return Some(Value::Int(*a.get(idx)?));
                }
                Value::LongArray(a) => {
                    let idx: usize = part.parse().ok()?;
                    return Some(Value::Long(*a.get(idx)?));
                }
                _ => return None,
            }
        }

        Some(current_node.clone())
    }

    pub fn set(&mut self, path: &str, value: Value) -> Option<bool> {
        let mut parts = path.split('.').peekable();
        let mut props = self.raw();
        let mut current_node = props.get_mut(parts.next()?)?;

        while let Some(part) = parts.next() {
            let is_last = parts.peek().is_none();

            match current_node {

                Value::ByteArray(a) => {
                    if !is_last { return None };
                    let idx: usize = part.parse().ok()?;
                    match value {
                        Value::Byte(gv) => { a[idx] = gv; }
                        _ => { return None; }
                    }
                }
                Value::IntArray(a) => {
                    if !is_last { return None };
                    let idx: usize = part.parse().ok()?;
                    match value {
                        Value::Int(gv) => { a[idx] = gv; }
                        _ => { return None; }
                    }
                }
                Value::LongArray(a) => {
                    if !is_last { return None };
                    let idx: usize = part.parse().ok()?;
                    match value {
                        Value::Long(gv) => { a[idx] = gv; }
                        _ => { return None; }
                    }
                }
                Value::List(a) => {
                    let idx: usize = part.parse().ok()?;
                    if is_last {
                        a[idx] = value;
                        return Some(true);
                    }
                    current_node = a.get_mut(idx)?;
                }
                Value::Compound(m) => {
                    if is_last {
                        m.insert(part.parse().unwrap(), value);
                        return Some(true);
                    }
                    current_node = m.get_mut(part)?;
                }
                _ => { Some(current_node.clone()); }
            }
        }

        Some(true)
    }
}

impl PartialEq for Properties {
    fn eq(&self, other: &Self) -> bool {
        if Arc::ptr_eq(&self.props, &other.props) { return true }

        let self_props = self.props.lock().unwrap();
        let other_props = other.props.lock().unwrap();
        *self_props == *other_props
    }
}
