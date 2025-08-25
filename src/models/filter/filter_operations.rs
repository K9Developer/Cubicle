use crate::models::filter::filter::ComparableValue;

#[derive(Clone, Copy, Debug)]
pub enum FilterOperation {
    Equals,
    LessThan,
    GreaterThan,
    LessThanEquals,
    GreaterThanEquals,
    Contains,
    Within
}

impl FilterOperation {
    pub fn eval(&self, a: &ComparableValue, b: &ComparableValue) -> Option<bool> {
        match self {
            FilterOperation::Equals             => Some(a == b),
            FilterOperation::LessThan           => Some(a <  b),
            FilterOperation::GreaterThan        => Some(a >  b),
            FilterOperation::LessThanEquals     => Some(a <= b),
            FilterOperation::GreaterThanEquals  => Some(a >= b),
            FilterOperation::Contains => match (a, b) {
                (ComparableValue::Text(s), ComparableValue::Text(sub)) => Some(s.contains(sub)),
                (ComparableValue::List(list), v) => Some(list.iter().any(|x| x == v)),
                (ComparableValue::Map(map), v) => Some(map.keys().any(|x| match v {
                    ComparableValue::Text(vs) => vs == x,
                    _ => false,
                })),
                _ => None,
            },
            FilterOperation::Within => match (a, b) {
                (ComparableValue::Text(sub), ComparableValue::Text(s)) => Some(s.contains(sub)),
                (ComparableValue::Position(p), ComparableValue::List(poses)) => unimplemented!(), // within a box TODO
                _ => None
            }
        }
    }
}