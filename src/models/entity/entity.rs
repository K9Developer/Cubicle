use crate::models::other::inventory::Inventory;
use crate::models::world::block::Block;

// TODO: Actually complete the Entities

#[derive(Debug)]
struct GenericEntity {
    name: String
}

#[derive(Debug)]
struct BlockEntity {
    base: GenericEntity,
    block: Block
}

#[derive(Debug)]
struct PlayerEntity {
    base: GenericEntity,
    inventory: Inventory
}

#[derive(Debug)]
struct NormalEntity {
    base: GenericEntity,
    inventory: Inventory
}

#[derive(Debug)]
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
