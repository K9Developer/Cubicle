use std::cmp::PartialEq;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::constants::versions::Version;
use crate::loaders::loader::Loader;
use crate::models::other::region::{Region, RegionType};
use crate::models::world::dimension::Dimension;
use crate::models::world::selection::{Selection, SelectionBuilder};
use crate::types::{RegionPosition, WorldType};
// TODO: When loading a world have a WorldInfo struct with readonly flag

pub struct World<'a> {
    path: PathBuf,
    seed: u64,

    version: Arc<Version>,
    loader: Loader<'a>,

    dimensions: HashMap<String, Dimension>,
    unloaded_regions: Vec<Region>,

    self_ref: Option<WorldType<'a>>
}


// Generic World API
impl<'a> World<'a> {
    pub fn new(path: PathBuf, version: Arc<Version>) -> WorldType<'a> {
        let arc = Arc::new_cyclic(|weak_self| {
            Mutex::new(
                Box::from(World {
                    path,
                    seed: 0,
                    dimensions: HashMap::new(),
                    unloaded_regions: Vec::new(),
                    loader: Loader::new(version.clone()),
                    version,
                    self_ref: None,
                })
            )
        });

        {
            let mut world = arc.lock().unwrap();
            world.self_ref = Some(arc.clone());
        }

        arc
    }

    pub fn dimensions(&self) -> &HashMap<String, Dimension> { &self.dimensions }
    pub fn dimension(&self, name: &str) -> Option<&Dimension> { self.dimensions.get(name) }
    pub fn dimension_mut(&mut self, name: &str) -> Option<&mut Dimension> { self.dimensions.get_mut(name) }
    pub fn seed(&self) -> u64 { self.seed }
    pub fn path(&self) -> &PathBuf { &self.path }
    pub fn loader(&self) -> &Loader<'a> { &self.loader }
    pub fn version(&self) -> Arc<Version> { self.version.clone() }
    pub fn get(&self) -> WorldType<'a> { self.self_ref.clone().unwrap() }
    pub fn select<'r>(&'r mut self) -> Selection<'r, 'a> {
        SelectionBuilder::new_owned(self, self.version.clone()).all_chunks().build()
    }

    pub fn set_seed(&mut self, seed: u64) { self.seed = seed; }
    pub fn set_dimension(&mut self, name: String, dimension: Dimension) { self.dimensions.insert(name, dimension); }
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
        let regions: Vec<_> = self.unloaded_regions.iter()
            .filter(|region| position == region.position)
            .cloned()
            .collect();

        for region in regions {
            match region.region_type {
                RegionType::Block => {
                    // println!("Loading block region");
                    let (chunks, new_structures) = self.loader.block_loader().parse_region(&region);
                    let dim = self.dimensions.get_mut(region.position.dimension()).unwrap();
                    dim.set_chunks(chunks);
                    dim.structure_store_mut().add_structures(new_structures);
                }
                RegionType::Entity => {
                    // println!("Loading entity region");
                    let chunks_entities = self.loader().entity_loader().parse_region(&region);
                    let dim = self.dimensions.get_mut(region.position.dimension()).unwrap();
                    for (chunk_pos, chunk_entities) in chunks_entities {
                        let entity_keys = dim.entity_store_mut().add_entities(chunk_entities);
                        dim.chunk_mut(chunk_pos).unwrap().lock().unwrap().set_entities(entity_keys);
                    }
                }
            }
        }
    }
}

// TODO: separate this to somewhere else
pub trait WithLock<T> {
    fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R;
    fn with_read<R>(&self, f: impl FnOnce(&T) -> R) -> R;
}

impl<T> WithLock<T> for Mutex<T> {
    fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut g = self.lock().unwrap();
        f(&mut *g)
    }
    fn with_read<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        let g = self.lock().unwrap();
        f(&*g)
    }
}

impl<T> WithLock<T> for Arc<Mutex<T>> {
    fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut g = self.lock().unwrap();
        f(&mut *g)
    }

    fn with_read<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        let g = self.lock().unwrap();
        f(&*g)
    }
}