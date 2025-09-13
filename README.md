<h1 align="center" id="title">Cubicle</h1>

<p align="center"><img src="https://socialify.git.ci/K9Developer/Cubicle/image?font=Inter&amp;language=1&amp;name=1&amp;owner=1&amp;pattern=Plus&amp;theme=Dark" alt="project-image"></p>

<p id="description">The universal minecraft world parser and editor. (Still WIP)</p>

<p align="center"><img src="https://img.shields.io/github/last-commit/K9Developer/Cubicle" alt="shields"><img src="https://img.shields.io/github/issues/K9Developer/Cubicle" alt="shields"><img src="https://img.shields.io/github/license/K9Developer/Cubicle" alt="shields"><img src="https://img.shields.io/badge/version-0.7v-blue" alt="shields"></p>

<img src="https://raw.githubusercontent.com/K9Developer/Cubicle/refs/heads/master/cubicle_logo_sm.png" alt="project-screenshot" width="400" height="400">


<h2>Usage</h2>

```rust

fn main() {
    
    // Create world object
    let world_path = "...";
    let version = VersionManager::get("1.20.1", WorldKind::Singleplayer);
    let world = World::new(world_path.parse().unwrap(), version);

    // Register all regions and parse the region at 0 0
    world.with(|w| {
        let region_position = RegionPosition::new(0, 0, "overworld");

        w.register_regions();
        w.load_region(region_position);
    });


    // Create a filter that will catch stone blocks that their X value is <= than 3
    let block_filter = Filter::And(vec![
        Filter::Compare(FilterKey::ID, FilterOperation::Equals, "minecraft:stone".into()),
        Filter::Compare(FilterKey::X_POSITION, FilterOperation::LessThanEquals, 3.into()),
    ]);

   world.with(|w| {
       
       // Create selection of world (this way we edit and do more complicated operations on worlds)
       let mut selection = w.select();
       
       // Find the blocks using the filter and a callback
       selection.find_blocks(block_filter, |mut matching_block| {
           
           // Set the matching block to a redstone block and commit the changes to the world
           matching_block.set_id("minecraft:redstone_block");
           matching_block.commit();
           true
       });
   })
}
```
  
<h2>🧐 Features</h2>

Here're some of the project's best features:

*   Load any minecraft world (any version)
*   Modify the world data
*   Save the world as any version
*   Filter for any block or entity
*   Tick the world to simulate it
*   Compress the world to very low sizes
*   And More!

<h2>🛠️ Installation Steps:</h2>

<p>1. Clone the repository</p>

```
git clone https://github.com/K9Developer/Cubicle
```

<p>2. Build The Project</p>

```
cargo build
```

<p>3. Code some tools</p>

<h2>🍰 Contribution Guidelines:</h2>

Incase you'd like to contribute fork the repository make changes and then create a pull request. I'd suggest first creating an issue where we could discuss details about the features / fixes you'd like to add before coding it up :)

  
  
<h2>💻 Built with</h2>

Technologies used in the project:

<h2>🛡️ License:</h2>

This project is licensed under the

<h2>💖Like my work?</h2>

If you'd like to donate feel free to support me on https://ko-fi.com/ilaik
