use std::collections::HashMap;
use crate::models::world::dimension::Dimension;

struct World<'a> {
    seed: u64,
    dimensions: HashMap<String, Dimension<'a>>
}

impl<'a> World<'a> {
    fn new(seed: u64) -> World<'a> {
        World {
            seed,
            dimensions: HashMap::new()
        }
    }

    fn dimensions(&self) -> &HashMap<String, Dimension<'a>> { &self.dimensions }
    fn dimension(&mut self, name: &str) -> &mut Dimension<'a> { self.dimensions.get_mut(name).unwrap() }
    fn seed(&self) -> u64 { self.seed }

    fn set_seed(&mut self, seed: u64) { self.seed = seed; }
    fn set_dimension(&mut self, name: String, dimension: Dimension<'a>) { self.dimensions.insert(name, dimension); }
}