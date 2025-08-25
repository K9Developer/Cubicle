pub mod constants {
    pub mod config;
    pub mod constants;
    pub mod versions;
}
pub mod extensions {
    pub mod test;
}
pub mod loaders {
    pub mod blocks_loader {
        pub mod loader;
        pub mod v3465;
    }
    pub mod entities_loader {
        pub mod loader;
        pub mod v3465;
    }
    pub mod loader;
    pub mod utils;
}
pub mod models {
    pub mod entity {
        pub mod block_entity;
        pub mod entity;
    }
    pub mod filter {
        pub mod comparable_value;
        pub mod filter;
        pub mod filter_keys;
        pub mod filter_operations;
    }
    pub mod nbt_structures {
        pub mod v3465 {
            pub mod entities;
            pub mod regular;
        }
    }
    pub mod other {
        pub mod fast_set;
        pub mod inventory;
        pub mod position;
        pub mod properties;
        pub mod region;
        pub mod tick;
    }
    pub mod stores {
        pub mod biome_store;
        pub mod block_store;
        pub mod structure_store;
    }
    pub mod world {
        pub mod block;
        pub mod chunk;
        pub mod dimension;
        pub mod world;
    }
    pub mod world_structures {
        pub mod generic_structure;
    }
    pub mod managers;
}
pub mod traits {
    pub mod access {
        pub mod blocks;
        pub mod prelude;
    }
    pub mod misc {
        pub mod store;
    }
}
pub mod utils;
