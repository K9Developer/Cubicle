pub use crate::models::filter::comparable_value::ComparableValue;
use crate::models::filter::filter_operations::FilterOperation;

trait Filterable {
    fn _value_by_filter_path(path: &str) -> ComparableValue;
    fn _evaluate_filter(filter: Filter) -> bool;
}

#[derive(Clone, Debug)]
pub enum Filter {
    Compare(String, FilterOperation, ComparableValue),
    And(Vec<Filter>),
    Or(Vec<Filter>),
    Not(Box<Filter>),
}

impl Filter {
    fn from_filter_string(filter_string: &str) -> Filter {
        // pos within
        unimplemented!()
    }
}