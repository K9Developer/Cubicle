use crate::models::block_entity::types::command_block::CommandBlockBlockEntity;
use crate::models::block_entity::types::cooker::cooker::CookerBlockEntity;
use crate::models::block_entity::types::lectern::LecternBlockEntity;
use crate::models::block_entity::types::sign::SignBlockEntity;
use crate::models::block_entity::types::spawner::SpawnerBlockEntity;
use crate::models::block_entity::types::storage_container::storage_container::StorageContainerBlockEntity;
use crate::models::other::properties::Properties;
use crate::models::positions::whole_position::Position;
use crate::traits::block_entity::BlockEntityTrait;

#[derive(Debug)]
pub struct GenericBlockEntity {
    id: String,
    position: Position,
    properties: Properties
}

impl GenericBlockEntity {
    pub fn new(id: String, position: Position, properties: Properties) -> Self {
        GenericBlockEntity {
            id, position, properties
        }
    }

    pub fn id(&self) -> &str { &self.id }
    pub fn position(&self) -> &Position { &self.position }
    pub fn properties(&self) -> &Properties { &self.properties }
    pub fn properties_mut(&mut self) -> &mut Properties { &mut self.properties }

    pub fn set_properties(&mut self, properties: Properties) { self.properties = properties }
    pub fn set_position(&mut self, position: Position) { self.position = position }
    pub fn set_id(&mut self, id: String) { self.id = id }
}

#[derive(Debug)]
pub enum BlockEntity {
    StorageContainer(StorageContainerBlockEntity),
    Cooker(CookerBlockEntity),
    Lectern(LecternBlockEntity),
    Spawner(SpawnerBlockEntity),
    CommandBlock(CommandBlockBlockEntity),
    Sign(SignBlockEntity),
    Other(GenericBlockEntity),
}

impl BlockEntityTrait for BlockEntity {
    fn base(&self) -> &GenericBlockEntity {
        match self {
            BlockEntity::StorageContainer(a) => a.base(),
            BlockEntity::Cooker(a) => a.base(),
            BlockEntity::Lectern(a) => a.base(),
            BlockEntity::Spawner(a) => a.base(),
            BlockEntity::CommandBlock(a) => a.base(),
            BlockEntity::Sign(a) => a.base(),
            BlockEntity::Other(a) => a,
        }
    }

    fn base_mut(&mut self) -> &mut GenericBlockEntity {
        match self {
            BlockEntity::StorageContainer(a) => a.base_mut(),
            BlockEntity::Cooker(a) => a.base_mut(),
            BlockEntity::Lectern(a) => a.base_mut(),
            BlockEntity::Spawner(a) => a.base_mut(),
            BlockEntity::CommandBlock(a) => a.base_mut(),
            BlockEntity::Sign(a) => a.base_mut(),
            BlockEntity::Other(a) => a,
        }
    }
}

impl BlockEntity {
    pub fn as_storage(&self) -> Option<&StorageContainerBlockEntity> {
        match self {
            BlockEntity::StorageContainer(s) => Some(s), _ => None
        }
    }
    pub fn as_storage_mut(&mut self) -> Option<&mut StorageContainerBlockEntity> {
        match self {
            BlockEntity::StorageContainer(s) => Some(s), _ => None
        }
    }

    pub fn as_cooker(&self) -> Option<&CookerBlockEntity> {
        match self {
            BlockEntity::Cooker(s) => Some(s), _ => None
        }
    }
    pub fn as_cooker_mut(&mut self) -> Option<&mut CookerBlockEntity> {
        match self {
            BlockEntity::Cooker(s) => Some(s), _ => None
        }
    }

    pub fn as_spawner(&self) -> Option<&SpawnerBlockEntity> {
        match self {
            BlockEntity::Spawner(s) => Some(s), _ => None
        }
    }
    pub fn as_spawner_mut(&mut self) -> Option<&mut SpawnerBlockEntity> {
        match self {
            BlockEntity::Spawner(s) => Some(s), _ => None
        }
    }

    pub fn as_command_block(&self) -> Option<&CommandBlockBlockEntity> {
        match self {
            BlockEntity::CommandBlock(c) => Some(c), _ => None
        }
    }
    pub fn as_command_block_mut(&mut self) -> Option<&mut CommandBlockBlockEntity> {
        match self {
            BlockEntity::CommandBlock(c) => Some(c), _ => None
        }
    }

    pub fn as_sign(&self) -> Option<&SignBlockEntity> {
        match self {
            BlockEntity::Sign(s) => Some(s), _ => None
        }
    }
    pub fn as_sign_mut(&mut self) -> Option<&mut SignBlockEntity> {
        match self {
            BlockEntity::Sign(s) => Some(s), _ => None
        }
    }
}
