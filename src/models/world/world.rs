use std::cmp::PartialEq;
use std::collections::HashMap;
use std::path::PathBuf;
use crate::constants::versions::Version;
use crate::loaders::loader::{get_loader, Loader};
use crate::models::other::position::Position;
use crate::models::other::region::Region;
use crate::models::world::dimension::Dimension;

pub struct World<'a> {
    path: PathBuf,

    seed: u64,
    dimensions: HashMap<String, Dimension<'a>>,

    unloaded_regions: Vec<Region>,
    version: &'a Version,
    world_loader: Box<dyn Loader<'a> + 'a>
}

#[derive(Debug, PartialEq, Eq)]
pub enum WorldType {
    SINGLEPLAYER,
    MULTIPLAYER,
}


impl<'a> World<'a> {
    pub fn new(path: PathBuf, version: &'a Version) -> Box<World<'a>> {
        Box::from(World {
            path,
            seed: 0,
            dimensions: HashMap::new(),
            unloaded_regions: Vec::new(),
            world_loader: get_loader(version),
            version,
        })
    }

    pub fn dimensions(&self) -> &HashMap<String, Dimension<'a>> { &self.dimensions }
    pub fn dimension(&mut self, name: &str) -> &mut Dimension<'a> { self.dimensions.get_mut(name).unwrap() }
    pub fn seed(&self) -> u64 { self.seed }
    pub fn path(&self) -> &PathBuf { &self.path }
    pub fn loader(&self) -> &Box<dyn Loader<'a> + 'a> { &self.world_loader }

    pub fn set_seed(&mut self, seed: u64) { self.seed = seed; }
    pub fn set_dimension(&mut self, name: String, dimension: Dimension<'a>) { self.dimensions.insert(name, dimension); }
    pub fn set_unloaded_regions(&mut self, unloaded_regions: Vec<Region>) { self.unloaded_regions = unloaded_regions }

    // Load related
    fn register_regions(&mut self) {
        self.unloaded_regions = self.world_loader.get_region_files(self.path.clone());
    }
    pub fn load_region(&'a mut self, position: Position) {
        for region in self.unloaded_regions.iter() {
            if position == region.position {
                let chunks = self.world_loader.parse_region(region);
                let dim = self.dimensions.get_mut(region.position.dimension()).unwrap();
                dim.set_chunks(chunks);
                break;
            }
        }
    }

}