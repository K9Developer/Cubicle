use std::collections::HashMap;
use fastnbt::Value;
use cubicle::constants::versions::Version;

fn main() {

    let a = Value::Compound(HashMap::from([
        ("a".to_string(), Value::String("A".to_string())),
        ("b".to_string(), Value::String("B".to_string())),
    ]));
    let b = Value::Compound(HashMap::from([
        ("b".to_string(), Value::String("B".to_string())),
        ("a".to_string(), Value::String("A".to_string())),
    ]));

    println!("{}", a == b);
}
