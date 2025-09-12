use crate::models::entity::entity::Entity;
pub use crate::models::filter::comparable_value::ComparableValue;
use crate::models::filter::filter_operations::FilterOperation;
use crate::models::filter::local_structure::LocalStructure;
use crate::models::world::fulls::full_block::FullBlock;
use crate::models::world::fulls::full_entity::FullEntity;

#[derive(Clone, Debug)]
pub enum Filter<'a> {
    Compare(&'a str, FilterOperation, ComparableValue),
    And(Vec<Filter<'a>>),
    Or(Vec<Filter<'a>>),
    Not(Vec<Filter<'a>>),

    LocalStructure(&'a LocalStructure)
}

impl<'a> Filter<'a> {
    pub fn from_filter_string(filter_string: &str) -> Filter<'a> {
        unimplemented!()
    }

    fn block_key_to_value(k: &str, b: &FullBlock) -> ComparableValue {
        match k {
            "key:x" => ComparableValue::Int(b.position().x() as i64),
            "key:y" => ComparableValue::Int(b.position().y() as i64),
            "key:z" => ComparableValue::Int(b.position().z() as i64),
            "key:pos" => ComparableValue::Position(b.position().clone()),
            "key:id" => ComparableValue::Text(b.id().to_owned()),
            _ => {
                match b.properties().get(k) {
                    Some(p) => ComparableValue::from_nbt_value(p),
                    None => { ComparableValue::Null }
                }
            }
        }
    }

    fn entity_key_to_value(k: &str, e: &FullEntity) -> ComparableValue {
        match k {
            "key:x" => ComparableValue::Int(e.entity().base().position().x() as i64),
            "key:y" => ComparableValue::Int(e.entity().base().position().y() as i64),
            "key:z" => ComparableValue::Int(e.entity().base().position().z() as i64),
            "key:pos" => ComparableValue::EntityPosition(e.entity().base().position().clone()),
            "key:id" => {
                match e.entity() {
                    Entity::Player(_) => { todo!() } // Need to do Player logic
                    Entity::Mob(m) => ComparableValue::Text(m.id().to_owned())
                }
            },
            _ => {
                let props = {
                    match e.entity() {
                        Entity::Player(p) => { todo!() } // Need to do Player logic
                        Entity::Mob(m) => { m.properties() }
                    }
                };

                match props.get(k) {
                    Some(p) => ComparableValue::from_nbt_value(p),
                    None => { ComparableValue::Null }
                }
            }
        }
    }

    pub fn matches_block(&self, block: &FullBlock) -> bool {
        match self {
            Filter::Compare(key, op, val) => {
                let actual_val = Filter::block_key_to_value(key, block);
                let result = op.eval(val, &actual_val);
                if let Some(result) = result {
                    result
                } else {
                    // bad evaluation // TODO: better handling?
                    false
                }
            }
            Filter::And(filters) => {
                for filter in filters {
                    if !filter.matches_block(&block) { return false; }
                }
                true
            }
            Filter::Or(filters) => {
                for filter in filters {
                    if filter.matches_block(&block) { return true; }
                }
                false
            }
            Filter::Not(filters) => {
                for filter in filters {
                    if filter.matches_block(&block) { return false; }
                }
                true
            }
            Filter::LocalStructure(_) => { todo!() }
        }
    }

    pub fn matches_entity(&self, entity: &FullEntity) -> bool {
        match self {
            Filter::Compare(key, op, val) => {
                let actual_val = Filter::entity_key_to_value(key, entity);
                let result = op.eval(val, &actual_val);
                if let Some(result) = result {
                    result
                } else {
                    // bad evaluation // TODO: better handling?
                    return false;
                }
            }
            Filter::And(filters) => {
                for filter in filters {
                    if !filter.matches_entity(entity) { return false; }
                }
                true
            }
            Filter::Or(filters) => {
                for filter in filters {
                    if filter.matches_entity(entity) { return true; }
                }
                false
            }
            Filter::Not(filters) => {
                for filter in filters {
                    if filter.matches_entity(entity) { return false; }
                }
                true
            }
            _ => false
        }
    }
}