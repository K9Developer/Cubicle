use cubicle::constants::versions::{VersionManager};
use cubicle::models::world::world::{World};
use std::time::Instant;
use serde::__private::de::IdentifierDeserializer;
use cubicle::models::filter::comparable_value::ComparableValue;
use cubicle::models::filter::filter::Filter;
use cubicle::models::filter::filter_keys::FilterKey;
use cubicle::models::filter::filter_operations::FilterOperation;
use cubicle::models::other::region::{Region, RegionType};
use cubicle::models::positions::chunk_position::ChunkPosition;
use cubicle::models::positions::whole_position::Position;
use cubicle::models::world::selection::{Selection, SelectionBuilder};
use cubicle::traits::access::prelude::{BlockReader, EntityReader};
use cubicle::traits::block_entity::BlockEntityTrait;
use cubicle::types::{HeightmapKind, RegionPosition, WorldKind};
use cubicle::utils::lock_utils::WithLock;
use cubicle::utils::position_utils::{chunk_offset_to_position, chunk_position_to_world_position, relative_position_to_world_position};
// TODO: Finish all todos before doing more versions!
/*
TODO: Some script to auto generate block states file (could GH actions)
// TODO: Make the entities be like the block entities in terms of categories.
enum Entity {
    Item(ItemEntity)
}

enum ItemEntity {
    Container(ContainerItem)
    Other(GenericItem)
}

struct ContainerItem {
    get_inv()
}

match entity {
    Entity::Item(i) => {
        ItemEntity::Container(c) => {
        }
    }
}


TODO: Writers - have dirty region list on each dimension and then world will have save_dirty_as() save_dirty() save_all_as() save_all()
TODO: In the future add an extension that will allow for more structure control like (pesudo) `(structure as Village).houses()` - it will be a wrapper around the generic structure and do in place calcs with cache (a bunch of wrappers)

TODO: Make sure biomes are exact - look at edge of biome and check

TODO: Parser optimizations - noticed creating chunk::new is the slowest operation by more than 3 times than everything together. Probably the stores allocating so much memory, figure out how to store more compact?
TODO: Load all needed things

TODO: Check all ingame block entities and add main ones like crafter

TODO: When saving a world with a sign for example that has both sides with text to a version lower that doesnt have that feature we should have the function return StripLog or somth that shows what was ignored

TODO: lock is an ItemPredicate (look at brewing stand lock on wiki), need to parse and do it
TODO: Allocate an arena and then chunk will use that. Somehow compress chunk. Each region is around 100 MB
TODO: Have some kind of config we can setup for version translations (what happens when situations)
*/

fn main() {S

    // Create world object
    let world_path = "C:/Users/ilaik/AppData/Roaming/.minecraft/saves/1_20_1 - Cubicle Test";
    let version = VersionManager::get("1.20.1", WorldKind::Singleplayer);
    let world = World::new(world_path.parse().unwrap(), version);

    // Register all regions and parse the region at 0 0
    world.with(|w| {
        let region_position = RegionPosition::new(0, 0, "overworld".into());

        w.register_regions();
        let s = Instant::now();
        w.load_region(region_position);
        let e = s.elapsed();
        println!("Took {:?}", e)
    });

    world.with(|w| {
        let b = w.select().block_at_position(Position::new("overworld".into(), 38, 81, 38)).unwrap();
        let a = b.data().unwrap();
        let x = a.as_cooker().unwrap();
        let i = x.as_furnace().unwrap();
        println!("{:?}", i);

    })


    // Create a filter that will catch stone blocks that their X value is <= than 3
   //  let block_filter = Filter::And(vec![
   //      Filter::Compare(FilterKey::ID, FilterOperation::Equals, "minecraft:stone".into()),
   //      Filter::Compare(FilterKey::X_POSITION, FilterOperation::LessThanEquals, 3.into()),
   //  ]);
   //
   // world.with(|w| {
   //
   //     // Create selection of world (this way we edit and do more complicated operations on worlds)
   //     let mut selection = w.select();
   //
   //     // Find the blocks using the filter and a callback
   //     selection.find_blocks(block_filter, |mut matching_block| {
   //
   //         // Set the matching block to a redstone block and commit the changes to the world
   //         matching_block.set_id("minecraft:redstone_block");
   //         matching_block.commit();
   //         true
   //     });
   // })
}


/*

TODO: ONLY WHEN NEEDED:
Make a script that runs when the porgram starts if no data,
it will download all the versions' jars and then generates reports to get the
block data, etc.

What we'll do is have something run online to store all the data, instead of doing it on machine to save some computation, etc
0. If block_states.rs doesnt have funcs it will say build first to generate funcs (or have warning if not latest)
1. build.rs takes latest version and will generate block_states.rs
2. write the data into constants/data/{version}.json
3. loading a version will load the file above, if no matching exists it will download and parse
4. we'll be able to do: version.block_states["facing_direction"].key
*/