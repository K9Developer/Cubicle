use std::collections::HashMap;
use fastnbt::Value;
use cubicle::models::world::block::Block;

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
