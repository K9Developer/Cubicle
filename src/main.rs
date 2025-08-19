use std::collections::HashMap;
use std::time::Instant;
use fastnbt::Value;
use cubicle::constants::versions::Version;
use cubicle::models::world::world::{World, WorldType};

fn main() {

    let v = Version::new("1.20.1", WorldType::SINGLEPLAYER);
    let w = World::new("C:/Users/ilaik/AppData/Roaming/.minecraft/saves/1_20_1 - Cubicle Test".parse().unwrap(), &v);
    let l = w.loader();
    let regions = l.get_region_files("C:/Users/ilaik/AppData/Roaming/.minecraft/saves/1_20_1 - Cubicle Test".parse().unwrap());

    println!("Loaded {} regions!", regions.len());
        let start = Instant::now();

    w.loader().parse_region(&regions[0]);
        let duration = start.elapsed();
    println!("Elapsed: {:?}", duration);

}
