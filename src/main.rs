use cubicle::constants::versions::{VersionManager};
use cubicle::models::world::world::{World, WorldKind};
use std::time::Instant;
use cubicle::models::filter::comparable_value::ComparableValue;
use cubicle::models::filter::filter::Filter;
use cubicle::models::filter::filter_keys::FilterKey;
use cubicle::models::filter::filter_operations::FilterOperation;
use cubicle::models::filter::local_structure::{LocalStructure, Offset};
use cubicle::models::positions::chunk_position::ChunkPosition;
use cubicle::models::world::block::Block;
use cubicle::models::world::selection::Selection;
use cubicle::traits::access::prelude::BlockAccess;
use cubicle::traits::access::prelude::{BlockReader};
use cubicle::types::{RegionPosition};
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

TODO: Too many version clones, not sure how to handle but it probably shouldnt be looking like this

have:
FullBlock which will have more info like biome, position, etc.
world.get_biome_at_position() -> &str
world.get_block_at_position() -> FullBlock
FullBlock will have a reference to world and then doing FullBlock.set_id("minecraft:stone") will actually replace the block
*/

/*
Up next:
    * Filters - We'll have a Filter that has key, comparer, value. for example:
     let f = Filter::new();
     f.add(FilterKey::x_pos, FilterComp::BiggerThan, {3})
     f.add(FilterKey::position, FilterComp::Within, {pos1, pos2})
    * Access Traits
    * The Managers (EntityManager, BlockManager, etc.)
    * The heightmaps
*/

fn main() {
    // let v = VersionManager::get("1.20.1", WorldType::SINGLEPLAYER);
    //
    // w.register_regions();
    // let start = Instant::now();
    // w.load_region(Position::new("overworld", -1f32, 0f32, -1f32));
    // let end = start.elapsed();
    // println!("Time elapsed in load_region() is: {:?}", end);
    //
    // let b = w.get_block_at_position(Position::new("overworld", -504f32, 62f32, -504f32));
    // println!("Block at position: {:?}", b);
    // // let biome = w.dimension("overworld").chunk((-32, -32)).unwrap().biome_store().get_biome_at_block_position(
    // //     Position::new("overworld", 0., 0., 0.)
    // // );
    // // println!("Biome: {:?}", biome);
    // //
    // // let dim = w.dimension("overworld");
    // // let all = dim.structure_store().structures();
    // // for s in all {
    // //     let b = s.chunk_position().to_block_coords(v.data.chunk_size);
    // //     println!("Found a structure with ID of {:?} at {} {} {}", s.id(), b.x(), b.y(), b.z());
    // // }
    // //
    // // let es = w.get_entities_of_id("minecraft:cow");
    // // for ent in es {
    // //     println!("A cow is at: {:?} {:?} {:?}", ent.base().position().x(), ent.base().position().y(), ent.base().position().z());
    // // }
    // let chunk = w.dimension("overworld").chunk((-32, -32)).unwrap().biome_store();
    // println!("Biome (should be birch): {:?}", chunk.get_biome_at_block_position(Position::new("overworld", 9., 3., 9.)));
    // println!("Biome (should be lush): {:?}", chunk.get_biome_at_block_position(Position::new("overworld", 9., 2., 9.)));

    // TODO: For filters add a Filter::LocalStructure which checks blocks around so can do like commented code below
    // TODO: Filters will run in a certain bounding box, if ran on world it will be everything, if on chunk, just the chunk, then can build a list of chunks to check on too
    let v = VersionManager::get("1.20.1", WorldKind::SINGLEPLAYER);
    let world = World::new("C:/Users/ilaik/AppData/Roaming/.minecraft/saves/1_20_1 - Cubicle Test".parse().unwrap(), v);
    world.lock().unwrap().register_regions();

    let mut s = Instant::now();
    world.lock().unwrap().load_region(RegionPosition::new(0, 0, "overworld"));
    let mut e = s.elapsed();
    println!("Time elapsed in load_region() is: {:?}", e);
    println!("Loaded {} chunks!", world.lock().unwrap().dimension("overworld").unwrap().len());

    let block_filter = Filter::And(vec![
        Filter::Compare(FilterKey::ID.into(), FilterOperation::Equals, ComparableValue::Text("minecraft:stone".into())),
        Filter::Compare(FilterKey::X_POSITION.into(), FilterOperation::Equals, 3.into()),
        Filter::Compare(FilterKey::Z_POSITION.into(), FilterOperation::Equals, 3.into()),
    ]);

    let mut selection = Selection::new(&world)
        .selection_add_chunk_position(ChunkPosition::new(0,0,"overworld"));
    println!("Created section");
    s = Instant::now();
    let bs = selection.find_blocks(block_filter, 0);
    e = s.elapsed();
    println!("Found {} stone blocks in the column in {:?}:", bs.len(), e);
    for b in bs {
        println!("\t{} at {}", b.id(), b.position());
    }


    /*



    let block_filter = Filter::And(vec![
        Filter::Compare(FilterKey::ID.into(), FilterOperation::Equals, ComparableValue::Text("minecraft:stone".into())),
        Filter::LocalStructure(ls)
    ]);

    let block = Selection::new(&world)
                        .add(chunk1)
                        .add(chunk2)
                        .add(chunk3)
                        .find_block(block_filter)

    let block = Selection::new(&world).all_chunks().find_block(block_filter)

    println!("{:?}", block_filter);
     */
}
// filter 28, filter keys