use cubicle::constants::versions::Version;
use cubicle::models::other::position::Position;
use cubicle::models::world::world::{World, WorldType};
use std::time::Instant;

// TODO: Finish all todos before doing more versions!

fn main() {
    let v = Version::new("1.20.1", WorldType::SINGLEPLAYER);
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
}
