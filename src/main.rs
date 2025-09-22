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

// TODO: block_entities
// TODO: Have way more specificity. For example (Entities (items and categories of items), block entities, etc.):

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
TODO: In the future add an extension that will allow for more structure control like (pesudo) `(structure as Village).houses()`

TODO: Have better entity types. Things with similar stuff. Like Item, Zombie / Skeleton, etc., Player, etc.

TODO: Make sure biomes are exact - look at edge of biome and check

TODO: Parser optimizations - noticed creating chunk::new is the slowest operation by more than 3 times than everything together. Probably the stores allocating so much memory, figure out how to store more compact?
TODO: Load all needed things
TODO: Block filter should filter for block entities too
TODO: Think about string pool since a lot of dup strings

TODO: PropertiesTranslator - translate properties to other versions
TODO: Crafter for block entity
TODO: Test changing chest inventory and saving (when changing the inv need to affect the properties too)

TODO: When saving a world with a sign for example that has both sides with text to a version lower that doesnt have that feature we should have the function return StripLog or somth that shows what was ignored
TODO: FullBlock should have attached_block_entity: Option<BlockEntity>
TODO: Some blocks have some props in BlockEntity and some in the PaletteBlock... should i keep it like that or have some types like in BlockEntity so there are helper funcs
*/

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

