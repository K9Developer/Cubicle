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
TODO: See what happens when a chunk contains at least 128 blocks (8 bits) as then we might have to use a global palette? https://minecraft.wiki/w/Java_Edition_protocol/Chunk_format
TODO: Biomes

TODO: In the future add an extension that will allow for more structure control like (pesudo) `(structure as Village).houses()`
*/


fn main() {
    let v = VersionManager::get("1.20.1", WorldType::SINGLEPLAYER);
    let mut w = World::new("C:/Users/ilaik/AppData/Roaming/.minecraft/saves/1_20_1 - Cubicle Test".parse().unwrap(), &v);

    w.register_regions();
    w.load_region(Position::new("overworld", -1f32, 0f32, -1f32));

    let b = w.get_block_at_position(Position::new("overworld", -504f32, 62f32, -504f32));
    println!("Block at position: {:?}", b);

    let dim = w.dimension("overworld");
    let all = dim.structure_store().structures();
    for s in all {
        let b = s.chunk_position().to_block_coords(v.data.chunk_size);
        println!("Found a structure with ID of {:?} at {} {} {}", s.id(), b.x(), b.y(), b.z());
    }

    let es = w.get_entities_of_id("minecraft:cow");
    for ent in es {
        println!("A cow is at: {:?} {:?} {:?}", ent.base().position().x(), ent.base().position().y(), ent.base().position().z());
    }
}
