use std::collections::HashMap;
use std::fmt;
use fastnbt::Value;

// TODO: Add proper errors, using an error struct
// TODO: Comparison of blocks is very slow since it has to check the whole extra thing - make some kind of hash?

#[derive(Clone)]
pub struct Block {
    name: String,
    extra: HashMap<String, Value>,
    null_flag: bool,
}

impl Block {
    pub fn new(name: &str, extra: Option<HashMap<String, Value>>) -> Block {
        // TODO: Enforce a namespace
        Block {
            name: if name.contains(":") { name.to_string() } else { ("minecraft:".to_owned() + name).to_owned() },
            extra: extra.unwrap_or_default(),
            null_flag: false,
        }
    }

    pub fn new_null() -> Block {
        Block {
            name: "cubicle:null_block".to_string(),
            extra: HashMap::new(),
            null_flag: true,
        }
    }

    pub fn name(&self) -> & str { &self.name }
    pub fn namespace(&self) -> & str { self.name.split(':').next().unwrap() }
    pub fn id(&self) -> & str { self.name.split(':').nth(1).unwrap() }
    pub fn properties(&self) -> &HashMap<String, Value> { &self.extra }
    pub fn mut_properties(&mut self) -> &mut HashMap<String, Value> { &mut self.extra }
    pub fn is_null(&self) -> bool { self.null_flag }
    pub fn property(&self, path: &str) -> Option<Value> {
        let mut parts = path.split('.').peekable();
        let mut current_node = self.properties().get(parts.next()?)?;

        while let Some(part) = parts.next() {
            let is_last = parts.peek().is_none();

            match current_node {

                Value::ByteArray(a) => {
                    if !is_last { return None };
                    let idx: usize = part.parse().ok()?;
                    return Some(Value::Byte(*a.get(idx)?))
                }
                Value::IntArray(a) => {
                    if !is_last { return None };
                    let idx: usize = part.parse().ok()?;
                    return Some(Value::Int(*a.get(idx)?))
                }
                Value::LongArray(a) => {
                    if !is_last { return None };
                    let idx: usize = part.parse().ok()?;
                    return Some(Value::Long(*a.get(idx)?))
                }
                Value::List(a) => {
                    let idx: usize = part.parse().ok()?;
                    current_node = a.get(idx)?;
                }
                Value::Compound(m) => {
                    current_node = m.get(part)?;
                }
                _ => { Some(current_node.clone()); }
            }
        }

        Some(current_node.clone())
    }

    pub fn set_property(&mut self, path: &str, value: Value) -> Option<bool> {
        let mut parts = path.split('.').peekable();
        let mut current_node = self.mut_properties().get_mut(parts.next()?)?;

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

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Block")
            .field("name", &self.name)
            .field("extra", &self.extra)
            .finish()
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block(name: {}, properties: {} entries)",
            self.name,
            self.extra.len()
        )
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.extra == other.extra
    }
}

impl Eq for Block {}


/// --------- TESTS ---------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use fastnbt::Value;

    #[test]
    fn create_block() {
        let b = Block::new("minecraft:air", None);
        println!("{:?}", b);
    }

    #[test]
    fn block_name_and_namespace() {
        let b = Block::new("minecraft2:air", None);
        assert_eq!("minecraft2:air", b.name());
        assert_eq!("air", b.id());
        assert_eq!("minecraft2", b.namespace());

        let b = Block::new("air", None);
        assert_eq!("minecraft:air", b.name());
        assert_eq!("air", b.id());
        assert_eq!("minecraft", b.namespace());
    }

    #[test]
    fn block_properties() {
        let mut b = Block::new(
            "test",
            Some(HashMap::from([
                ("id".to_string(), Value::String("minecraft:chest".into())),
                (
                    "items".to_string(),
                    Value::List(vec![
                        Value::Compound(HashMap::from([
                            ("slot".to_string(), Value::Int(0)),
                            ("id".to_string(), Value::String("minecraft:apple".into())),
                            ("count".to_string(), Value::Byte(5)),
                        ])),
                        Value::Compound(HashMap::from([
                            ("slot".to_string(), Value::Int(1)),
                            ("id".to_string(), Value::String("minecraft:diamond".into())),
                            ("count".to_string(), Value::Byte(3)),
                            (
                                "ench".to_string(),
                                Value::List(vec![Value::Compound(HashMap::from([
                                    ("id".to_string(), Value::String("minecraft:fortune".into())),
                                    ("lvl".to_string(), Value::Int(3)),
                                ]))]),
                            ),
                        ])),
                    ]),
                ),
            ])),
        );

        assert_eq!(
            b.property("items.1.ench.0.id"),
            Some(Value::String("minecraft:fortune".into()))
        );

        assert_eq!(
            b.set_property(
                "items.1.ench.0.id",
                Value::Compound(HashMap::from([(
                    "a".to_string(),
                    Value::String("minecraft:chest".into())
                )]))
            ),
            Some(true)
        );

        assert_eq!(
            b.property("items.1.ench.0.id.a"),
            Some(Value::String("minecraft:chest".into()))
        );
    }

    fn map_order_a() -> HashMap<String, Value> {
        let mut m = HashMap::new();
        m.insert("hp".into(), Value::Int(10));
        m.insert("id".into(), Value::String("minecraft:stone".into()));
        m
    }

    fn map_order_b() -> HashMap<String, Value> {
        let mut m = HashMap::new();
        m.insert("id".into(), Value::String("minecraft:stone".into()));
        m.insert("hp".into(), Value::Int(10));
        m
    }

    #[test]
    fn equals_ignores_map_insert_order() {
        let a = Block::new("minecraft:stone", Some(map_order_a()));
        let b = Block::new("minecraft:stone", Some(map_order_b()));
        assert_eq!(a, b);
    }

    #[test]
    fn equals_ignores_nested_compound_order() {
        let mut pos1 = HashMap::new();
        pos1.insert("x".to_string(), Value::Int(1));
        pos1.insert("y".to_string(), Value::Int(2));

        let mut pos2 = HashMap::new();
        pos2.insert("y".to_string(), Value::Int(2));
        pos2.insert("x".to_string(), Value::Int(1));

        let mut extra1 = HashMap::new();
        extra1.insert("pos".into(), Value::Compound(pos1));

        let mut extra2 = HashMap::new();
        extra2.insert("pos".into(), Value::Compound(pos2));

        let a = Block::new("minecraft:stone", Some(extra1));
        let b = Block::new("minecraft:stone", Some(extra2));
        assert_eq!(a, b);
    }

    #[test]
    fn not_equal_when_name_differs() {
        let a = Block::new("minecraft:stone", Some(map_order_a()));
        let b = Block::new("minecraft:dirt",  Some(map_order_a()));
        assert_ne!(a, b);
    }

    #[test]
    fn not_equal_when_extra_value_differs() {
        let a = Block::new("minecraft:stone", Some(map_order_a()));

        let mut changed = map_order_a();
        changed.insert("hp".into(), Value::Int(11));

        let b = Block::new("minecraft:stone", Some(changed));
        assert_ne!(a, b);
    }

    #[test]
    fn not_equal_when_extra_keys_differ() {
        let a = Block::new("minecraft:stone", Some(map_order_a()));

        let mut fewer = HashMap::new();
        fewer.insert("id".into(), Value::String("minecraft:stone".into()));

        let b = Block::new("minecraft:stone", Some(fewer));
        assert_ne!(a, b);
    }
}