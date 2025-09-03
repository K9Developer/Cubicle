use std::cmp::PartialEq;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::constants::versions::Version;
use crate::loaders::loader::Loader;
use crate::models::other::region::{Region, RegionType};
use crate::models::world::dimension::Dimension;
use crate::types::{RegionPosition, WorldType};
// TODO: When loading a world have a WorldInfo struct with readonly flag

pub struct World<'a> {
    path: PathBuf,
    seed: u64,

    version: Arc<Version>,
    loader: Loader<'a>,

    dimensions: HashMap<String, Dimension<'a>>,
    unloaded_regions: Vec<Region>
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum WorldKind {
    SINGLEPLAYER,
    MULTIPLAYER,
}


// Generic World API
impl<'a> World<'a> {
    pub fn new(path: PathBuf, version: Arc<Version>) -> WorldType<'a> {
        Arc::new(Mutex::new(Box::from(World {
            path,
            seed: 0,
            dimensions: HashMap::new(),
            unloaded_regions: Vec::new(),
            loader: Loader::new(version.clone()),
            version,
        })))
    }

    pub fn dimensions(&self) -> &HashMap<String, Dimension<'a>> { &self.dimensions }
    pub fn dimension(&self, name: &str) -> Option<&Dimension<'a>> { self.dimensions.get(name) }
    pub fn dimension_mut(&mut self, name: &str) -> Option<&mut Dimension<'a>> { self.dimensions.get_mut(name) }
    pub fn seed(&self) -> u64 { self.seed }
    pub fn path(&self) -> &PathBuf { &self.path }
    pub fn loader(&self) -> &Loader<'a> { &self.loader }
    pub fn version(&self) -> Arc<Version> { self.version.clone() }

    pub fn set_seed(&mut self, seed: u64) { self.seed = seed; }
    pub fn set_dimension(&mut self, name: String, dimension: Dimension<'a>) { self.dimensions.insert(name, dimension); }
    pub fn set_unloaded_regions(&mut self, unloaded_regions: Vec<Region>) { self.unloaded_regions = unloaded_regions }

 }

// Load related
impl<'a> World<'a> {

    pub fn register_regions(&mut self) -> usize {
        self.unloaded_regions = self.loader().block_loader().get_region_files(self.path.clone());
        self.unloaded_regions.extend(self.loader().entity_loader().get_region_files(self.path.clone()));

        self.dimensions = HashMap::from([ //TODO: Dont hardcode this
            ("overworld".to_string(), Dimension::new("overworld".to_string(), self.version.clone())),
            ("the_nether".to_string(), Dimension::new("the_nether".to_string(), self.version.clone())),
            ("the_end".to_string(), Dimension::new("the_end".to_string(), self.version.clone()))
        ]
        );
        self.unloaded_regions.len()
    }

    pub fn load_region(&mut self, position: RegionPosition) {
        for region in self.unloaded_regions.iter() {
            if position == region.position {
                match region.region_type {
                    RegionType::Block => {
                        println!("Loading block region");
                        let (chunks, new_structures) = self.loader.block_loader().parse_region(region);
                        let dim = self.dimensions.get_mut(region.position.dimension()).unwrap();
                        dim.set_chunks(chunks);
                        dim.structure_store_mut().add_structures(new_structures);
                    },
                    RegionType::Entity => {
                        println!("Loading entity region");
                        let entities = self.loader().entity_loader().parse_region(region);
                        let dim = self.dimensions.get_mut(region.position.dimension()).unwrap();
                        dim.add_entities(entities);
                    }
                }
            }
        }
    }

    // pub fn get_block_at_position(&self, position: Position) -> Option<Block> {
    //     let (chunk_coords, relative_coords) = position.to_chunk_coords(self.version.data.chunk_size);
    //     println!("Chunk coords {:?}, relative block coords {:?}, block index {:?}", chunk_coords, relative_coords, relative_coords.to_index(&self.version));
    //     let dim = self.dimensions.get(position.dimension());
    //     if dim.is_none() { return None }
    //     let chunk = dim.unwrap().chunk(chunk_coords);
    //     if chunk.is_none() { return None }
    //     chunk.unwrap().block_store().get_block_at_index(relative_coords.to_index(self.version))
    // }
    //
    // // Very temporary until selectors
    // pub fn get_entities_of_id(&self, id: &str) -> Vec<&Entity> {
    //     let mut selected_entities = Vec::<&Entity>::new();
    //     for dim in self.dimensions.values() {
    //         for entity in dim.entities() {
    //             match entity {
    //                 Entity::Player(player) => {},
    //                 Entity::Mob(mob) => {
    //                     if mob.id() == id {selected_entities.push(entity);}
    //                 }
    //             }
    //         }
    //     }
    //     selected_entities
    // }

}
