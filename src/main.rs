use cubicle::constants::versions::{VersionManager};
use cubicle::models::world::world::{World};
use std::time::Instant;
use cubicle::models::filter::comparable_value::ComparableValue;
use cubicle::models::filter::filter::Filter;
use cubicle::models::filter::filter_keys::FilterKey;
use cubicle::models::filter::filter_operations::FilterOperation;
use cubicle::models::other::region::{Region, RegionType};
use cubicle::models::positions::chunk_position::ChunkPosition;
use cubicle::models::positions::whole_position::Position;
use cubicle::models::world::selection::{Selection, SelectionBuilder};
use cubicle::traits::access::prelude::{BlockReader, EntityReader};
use cubicle::types::{HeightmapKind, RegionPosition, WorldKind};
use cubicle::utils::lock_utils::WithLock;
use cubicle::utils::position_utils::{chunk_offset_to_position, chunk_position_to_world_position, relative_position_to_world_position};
// TODO: Finish all todos before doing more versions!
/*

TODO: Writers - have dirty region list on each dimension and then world will have save_dirty_as() save_dirty() save_all_as() save_all()
TODO: In the future add an extension that will allow for more structure control like (pesudo) `(structure as Village).houses()`

TODO: Have better entity types. Things with similar stuff. Like Item, Zombie / Skeleton, etc., Player, etc.

TODO: Make sure biomes are exact - look at edge of biome and check

TODO: Parser optimizations - noticed creating chunk::new is the slowest operation by more than 3 times than everything together. Probably the stores allocating so much memory, figure out how to store more compact?

*/
/*

*/

fn main() {
    let world_path = "C:/Users/ilaik/AppData/Roaming/.minecraft/saves/1_20_1 - Cubicle Test";
    let version = VersionManager::get("1.20.1", WorldKind::Singleplayer);
    let world = World::new(world_path.parse().unwrap(), version);

    let mut chunks_loaded = 0;
    world.with(|w| {
        w.register_regions();

        w.load_region(RegionPosition::new(0, -1, "overworld"));

        chunks_loaded = w.dimension("overworld").unwrap().chunk_count();
    });


    let block_filter = Filter::Compare(FilterKey::ID, FilterOperation::Equals, "minecraft:redstone_block".into());

   world.with(|w| {
       let mut sel = SelectionBuilder::new_owned(w, w.version()).with_chunk_position(ChunkPosition::new(14, -7, "overworld")).build();
       let chunk = sel.chunk(ChunkPosition::new(14, -7, "overworld")).unwrap();


       chunk.with(|c| {
           let b1 = c.biome_store().get_biome_at_position(Position::new("overworld", 13, 97, 4)).unwrap();
           let b2 = c.biome_store().get_biome_at_position(Position::new("overworld", 13, 98, 4)).unwrap();
           println!("Should be plains: {}", b1);
           println!("Should be beach: {}", b2);
       })

   })
}

// plains - 237 97 -108
// beach - 237 98 -108