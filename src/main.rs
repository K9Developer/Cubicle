use cubicle::constants::versions::{VersionManager};
use cubicle::models::world::world::{WithLock, World};
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
use cubicle::utils::position_utils::chunk_offset_to_position;
// TODO: Finish all todos before doing more versions!
/*
TODO: Have a WorldContentManager that will be in charge of the API of the content of the world. like below:
World {
    content_manager: WorldContentManager
}

WorldContentManager {
    entity_manager: EntityManager,
    block_manager: BlockManager,
    structure_manager: StructureManager,
    heightmap_manager: HeightmapManager
}

TODO: For example: have set_block for chunk, dimension, and the WorldContentManager - then they'll each call each other after a bit of position tweaking.
TODO: have WorldMetadata like WorldContentManager

TODO: In the future add an extension that will allow for more structure control like (pesudo) `(structure as Village).houses()`

TODO: Have better entity types. Things with similar stuff. Like Item, Zombie / Skeleton, etc., Player, etc.
TODO: When taking chunk_nbt for blocks and structures try to make blocks return the chunk_nbt so we can move it to structures next instead of having a ref and then cloning stuff. OR move .structures into the structures and then its all going to work fine

TODO: Have traits like in the link for world, dimension, chunk and they each propagate down after position modification: https://chatgpt.com/s/t_68aa03ff5aa48191abc4158adf2e91ef

TODO: Make sure biomes are exact - look at edge of biome and check

*/

/*
Up next:
    * The Managers (EntityManager, BlockManager, etc.)
    * The heightmaps
*/

fn main() {
    let world_path = "C:/Users/ilaik/AppData/Roaming/.minecraft/saves/1_20_1 - Cubicle Test";
    let version = VersionManager::get("1.20.1", WorldKind::Singleplayer);
    let world = World::new(world_path.parse().unwrap(), version);

    let mut chunks_loaded = 0;
    world.with(|w| {
        w.register_regions();
        let s = Instant::now();
        w.load_region(RegionPosition::new(0, 0, "overworld"));
        let e = s.elapsed();
        println!("Loaded region in {:?}", e);
        chunks_loaded = w.dimension("overworld").unwrap().chunk_count();
    });

    println!("Loaded {} chunks!", chunks_loaded);

    world.with(|w| {
        let dim = w.dimension("overworld").unwrap();
        let chu = dim.chunk((0,0)).unwrap();
        let chunk = chu.lock().unwrap();
        println!("SkyExposed: {:?}", chunk.heightmap_store().get_kind(HeightmapKind::SkyExposed).get_highest_y_at_position(14, 10));
        println!("MotionBlockingNoLeaves: {:?}", chunk.heightmap_store().get_kind(HeightmapKind::MotionBlockingNoLeaves).get_highest_y_at_position(14, 10));
        println!("MotionBlocking: {:?}", chunk.heightmap_store().get_kind(HeightmapKind::MotionBlocking).get_highest_y_at_position(14, 10));
        println!("Ground: {:?}", chunk.heightmap_store().get_kind(HeightmapKind::Ground).get_highest_y_at_position(14, 10));
    });

    let block_filter = Filter::And(vec![
        Filter::Compare(FilterKey::ID.into(), FilterOperation::Equals, ComparableValue::Text("minecraft:stone".into())),
        Filter::Compare(FilterKey::X_POSITION.into(), FilterOperation::Equals, 3.into()),
        Filter::Compare(FilterKey::Z_POSITION.into(), FilterOperation::Equals, 3.into())
    ]);

    let entity_filter = Filter::Compare(FilterKey::ID.into(), FilterOperation::Equals, ComparableValue::Text("minecraft:item".into()));

    let mut selection = SelectionBuilder::new(&world)
        .with_chunk_position(ChunkPosition::new(0,0,"overworld"))
        .build();

    // println!("Blocks in column:");
    // selection.find_blocks(block_filter, |mut block| {
    //     println!("\t{} at {}", block.id(), block.position());
    //     true
    // });

    // selection.find_entities(
    //     entity_filter,
    //     |e| {
    //         println!("{:?}", e);
    //         true
    //     }
    // )
    // TODO: Seems like it searches blocks from y>0

}