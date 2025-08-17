use crate::models::other::inventory::Inventory;
use crate::models::world::block::Block;

// TODO: Actually complete the Entities

struct GenericEntity {
    name: String
}

struct BlockEntity {
    base: GenericEntity,
    block: Block
}

struct PlayerEntity {
    base: GenericEntity,
    inventory: Inventory
}

struct NormalEntity {
    base: GenericEntity,
    inventory: Inventory
}

pub enum Entity {
    Player(PlayerEntity),
    Block(BlockEntity),
    Normal(NormalEntity),
}

impl Entity {
    fn base(&self) -> &GenericEntity {
        match self {
            Entity::Player(p) => &p.base,
            Entity::Block(b)  => &b.base,
            Entity::Normal(n) => &n.base
        }
    }
    fn base_mut(&mut self) -> &mut GenericEntity {
        match self {
            Entity::Player(p) => &mut p.base,
            Entity::Block(b)  => &mut b.base,
            Entity::Normal(n) => &mut n.base
        }
    }
}
