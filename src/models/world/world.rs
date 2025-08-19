use std::cmp::PartialEq;
use std::collections::HashMap;
use std::path::PathBuf;
use crate::constants::versions::Version;
use crate::loaders::loader::{get_loader, Loader};
use crate::models::other::position::Position;
use crate::models::other::region::Region;
use crate::models::world::block::Block;
use crate::models::world::dimension::Dimension;

// TODO: When loading a world have a WorldInfo struct with readonly flag

pub struct World<'a> {
    path: PathBuf,
    seed: u64,

    version: &'a Version,
    world_loader: Box<dyn Loader<'a> + 'a>,

    dimensions: HashMap<String, Dimension<'a>>,
    unloaded_regions: Vec<Region>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WorldType {
    SINGLEPLAYER,
    MULTIPLAYER,
}


// Generic World API
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
}

// World Loaders Related
impl<'a> World<'a> {
    pub fn register_regions(&mut self) -> usize {
        self.unloaded_regions = self.world_loader.get_region_files(self.path.clone());
        self.dimensions = HashMap::from([ //TODO: Dont hardcode this
            ("overworld".to_string(), Dimension::new("overworld".to_string(), self.version)),
            ("the_nether".to_string(), Dimension::new("the_nether".to_string(), self.version)),
            ("the_end".to_string(), Dimension::new("the_end".to_string(), self.version))
        ]
        );
        self.unloaded_regions.len()
    }

    pub fn load_region(&mut self, position: Position) {
        for region in self.unloaded_regions.iter() {
            if position == region.position {
                let chunks = self.world_loader.parse_region(region);
                let dim = self.dimensions.get_mut(region.position.dimension()).unwrap();
                dim.set_chunks(chunks);
                break;
            }
        }
    }

    pub fn get_block_at_position(&self, position: Position) -> Option<Block> {
        let (chunk_coords, relative_coords) = position.to_chunk_coords(self.version.data.chunk_size);
        println!("Chunk coords {:?}, relative block coords {:?}, block index {:?}", chunk_coords, relative_coords, relative_coords.to_index(&self.version));
        let dim = self.dimensions.get(position.dimension());
        if dim.is_none() { return None }
        let chunk = dim.unwrap().chunk(chunk_coords);
        if chunk.is_none() { return None }
        chunk.unwrap().block_store().get_block_at_index(relative_coords.to_index(self.version))
    }


}