use std::cmp::PartialEq;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::constants::versions::Version;
use crate::loaders::loader::MainLoader;
use crate::models::entity::entity::PlayerEntity;
use crate::models::other::lasso_string::LassoString;
use crate::models::other::region::{Region, RegionType};
use crate::models::other::tick::Tick;
use crate::models::positions::chunk_position::ChunkPosition;
use crate::models::world::chunk::Chunk;
use crate::models::world::dimension::Dimension;
use crate::models::world::fulls::full_block::FullBlock;
use crate::models::world::selection::{Selection, SelectionBuilder};
use crate::types::{ChunkType, RegionPosition, WorldType};
// TODO: When loading a world have a WorldInfo struct with readonly flag

pub struct World<'a> {
    path: PathBuf,
    seed: u64,

    version: Arc<Version>,
    loader: MainLoader<'a>,

    dimensions: HashMap<LassoString, Dimension>,
    unloaded_regions: Vec<Region>,
    players: Vec<PlayerEntity>,

    self_ref: Option<WorldType<'a>>
}


// Generic World API
impl<'a> World<'a> {
    pub fn new(path: PathBuf, version: Arc<Version>) -> WorldType<'a> {
        let arc = Arc::new_cyclic(|weak_self| {
            Mutex::new(
                World {
                    path,
                    seed: 0,
                    dimensions: HashMap::new(),
                    unloaded_regions: Vec::new(),
                    loader: MainLoader::new(version.clone()),
                    version,
                    self_ref: None,
                }
            )
        });

        {
            let mut world = arc.lock().unwrap();
            world.self_ref = Some(arc.clone());
        }

        arc
    }

    pub fn dimensions(&self) -> &HashMap<LassoString, Dimension> { &self.dimensions }
    pub fn dimension(&self, name: &LassoString) -> Option<&Dimension> { self.dimensions.get(name) }
    pub fn dimension_mut(&mut self, name: &LassoString) -> Option<&mut Dimension> { self.dimensions.get_mut(name) }
    pub fn seed(&self) -> u64 { self.seed }
    pub fn path(&self) -> &PathBuf { &self.path }
    pub fn loader(&self) -> &MainLoader<'a> { &self.loader }
    pub fn version(&self) -> Arc<Version> { self.version.clone() }
    pub fn get(&self) -> WorldType<'a> { self.self_ref.clone().unwrap() }
    pub fn select<'r>(&'r mut self) -> Selection<'r, 'a> {
        SelectionBuilder::new_owned(self, self.version.clone()).all_chunks().build()
    }

    pub fn set_seed(&mut self, seed: u64) { self.seed = seed; }
    pub fn set_dimension(&mut self, name: LassoString, dimension: Dimension) { self.dimensions.insert(name, dimension); }
    pub fn set_unloaded_regions(&mut self, unloaded_regions: Vec<Region>) { self.unloaded_regions = unloaded_regions }

    pub fn set_chunk(&mut self, position: ChunkPosition, block: FullBlock) -> Option<ChunkType> {
        let chunk = Chunk::new(
            position,
            self.version.data.version_data,
            Tick::new(0),
            Tick::new(0),
            "minecraft:full".to_string(),
            &self.version
        );

        match self.dimension_mut(chunk.position().dimension()) {
            Some(dim) => {
                Some(dim.set_chunk(chunk))
            },
            None => { None }
        }
    }

    pub fn delete_chunk(&mut self, position: ChunkPosition) -> Option<ChunkType> {
        let dim = self.dimension_mut(position.dimension());
        dim?.delete_chunk(position.position())
    }

 }

// Load related
impl<'a> World<'a> {

    pub fn load(&mut self) {
        // TODO: here will dry load all regions etc.

        let pl = self.loader.player_loader();
        let paths = pl.get_player_files(self.path.clone());
        for path in paths {
            self.players.push(pl.parse_player(&path));
        }
    }

    pub fn register_regions(&mut self) -> usize {
        self.unloaded_regions = self.loader().block_loader().get_region_files(self.path.clone());
        self.unloaded_regions.extend(self.loader().entity_loader().get_region_files(self.path.clone()));

        self.dimensions = HashMap::from([ //TODO: Dont hardcode this
            ("overworld".into(), Dimension::new("overworld".into(), self.version.clone())),
            ("the_nether".into(), Dimension::new("the_nether".into(), self.version.clone())),
            ("the_end".into(), Dimension::new("the_end".into(), self.version.clone()))
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
