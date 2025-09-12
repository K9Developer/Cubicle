use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt;
use fastnbt::Value;
use crate::models::other::properties::Properties;
// TODO: Add proper errors, using an error struct
// TODO: Comparison of blocks is very slow since it has to check the whole extra thing - make some kind of hash?

#[derive(Clone)]
pub struct PaletteBlock {
    name: String,
    extra: Properties,
    null_flag: bool,
}

impl PaletteBlock {
    pub fn new(name: &str, extra: Option<HashMap<String, Value>>) -> PaletteBlock {
        PaletteBlock {
            name: name.to_string(),
            extra: Properties::new(extra.unwrap_or_default()),
            null_flag: false,
        }
    }

    pub fn new_null() -> PaletteBlock {
        PaletteBlock {
            name: "cubicle:null_block".to_string(),
            extra: Properties::new(HashMap::new()),
            null_flag: true,
        }
    }

    pub fn name(&mut self) -> & str {
        if self.name.contains(':') {
            &self.name
        } else {
            let full = format!("minecraft:{}", self.name);
            self.name = full;
            &self.name
        }
    }

    pub fn namespace(&self) -> & str { self.name.split(':').next().unwrap() }
    pub fn id(&self) -> & str { self.name.split(':').nth(1).unwrap() }
    pub fn properties(&self) -> &Properties { &self.extra }
    pub fn properties_mut(&mut self) -> &mut Properties { &mut self.extra }
    pub fn is_null(&self) -> bool { self.null_flag }
}

impl fmt::Debug for PaletteBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Block")
            .field("name", &self.name)
            .field("extra", &self.extra)
            .finish()
    }
}

impl fmt::Display for PaletteBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block(name: {})",
            self.name,
        )
    }
}

impl PartialEq for PaletteBlock {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.extra == other.extra
    }
}

impl Eq for PaletteBlock {}

impl From<&str> for PaletteBlock {
    fn from(t: &str) -> Self {
        Self {
            name: t.to_string(),
            extra: Properties::new(HashMap::new()),
            null_flag: false,
        }
    }
}

/// --------- TESTS ---------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::time::Instant;
    use fastnbt::Value;

    #[test]
    fn create_block() {
        let b = PaletteBlock::new("minecraft:air", None);
        println!("{:?}", b);
    }

    #[test]
    fn block_name_and_namespace() {
        let mut b = PaletteBlock::new("minecraft2:air", None);
        assert_eq!("minecraft2:air", b.name());
        assert_eq!("air", b.id());
        assert_eq!("minecraft2", b.namespace());

        let mut b = PaletteBlock::new("air", None);
        assert_eq!("minecraft:air", b.name());
        assert_eq!("air", b.id());
        assert_eq!("minecraft", b.namespace());
    }

    #[test]
    fn block_properties() {
        let mut b = PaletteBlock::new(
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

        let mut props = b.properties_mut();

        assert_eq!(
            props.get("items.1.ench.0.id"),
            Some(Value::String("minecraft:fortune".into()))
        );

        assert_eq!(
            props.set(
                "items.1.ench.0.id",
                Value::Compound(HashMap::from([(
                    "a".to_string(),
                    Value::String("minecraft:chest".into())
                )]))
            ),
            Some(true)
        );

        assert_eq!(
            props.get("items.1.ench.0.id.a"),
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
        let a = PaletteBlock::new("minecraft:stone", Some(map_order_a()));
        let b = PaletteBlock::new("minecraft:stone", Some(map_order_b()));
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

        let a = PaletteBlock::new("minecraft:stone", Some(extra1));
        let b = PaletteBlock::new("minecraft:stone", Some(extra2));
        assert_eq!(a, b);
    }

    #[test]
    fn not_equal_when_name_differs() {
        let a = PaletteBlock::new("minecraft:stone", Some(map_order_a()));
        let b = PaletteBlock::new("minecraft:dirt", Some(map_order_a()));
        assert_ne!(a, b);
    }

    #[test]
    fn not_equal_when_extra_value_differs() {
        let a = PaletteBlock::new("minecraft:stone", Some(map_order_a()));

        let mut changed = map_order_a();
        changed.insert("hp".into(), Value::Int(11));

        let b = PaletteBlock::new("minecraft:stone", Some(changed));
        assert_ne!(a, b);
    }

    #[test]
    fn not_equal_when_extra_keys_differ() {
        let a = PaletteBlock::new("minecraft:stone", Some(map_order_a()));

        let mut fewer = HashMap::new();
        fewer.insert("id".into(), Value::String("minecraft:stone".into()));

        let b = PaletteBlock::new("minecraft:stone", Some(fewer));
        assert_ne!(a, b);
    }
}