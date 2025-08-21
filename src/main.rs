use cubicle::constants::versions::{VersionManager};
use cubicle::models::other::position::Position;
use cubicle::models::world::world::{World, WorldType};
use std::time::Instant;

// TODO: Finish all todos before doing more versions!

/*
TODO: Have a WorldContentManager that will be in charge of the API of the content of the world.
TODO: For example: have set_block for chunk, dimension, and the WorldContentManager - then they'll each call each other after a bit of position tweaking.
TODO: have WorldMetadata like WorldContentManager
TODO: Have a Properties struct so it could be used in entities, blocks, items, etc. It should include the get and set props with path
TODO: Make the get and set props macros (or use macros) so they just make code at the end without the whole looping and everything - itll be faster.
*/

fn main() {
    // let v = Version::new("1.20.1", WorldType::SINGLEPLAYER);
    let v = VersionManager::get("1.20.1", WorldType::SINGLEPLAYER);
    let mut w = World::new(
        "C:/Users/ilaik/AppData/Roaming/.minecraft/saves/1_20_1 - Cubicle Test"
            .parse()
            .unwrap(),
        &v,
    );

    w.register_regions();
    let start = Instant::now();
    w.load_region(Position::new("overworld", -1f32, 0f32, -1f32));
    let duration = start.elapsed();
    println!("Elapsed: {:?}", duration);
    let b = w.get_block_at_position(Position::new("overworld", -504f32, 62f32, -504f32));
    println!("Block at position: {:?}", b);
    let es = w.get_entities_of_id("minecraft:cow");
    for ent in es {
        println!("A cow is at: {:?} {:?} {:?}", ent.base().position().x(), ent.base().position().y(), ent.base().position().z());
    }
}
