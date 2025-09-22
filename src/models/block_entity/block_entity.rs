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
    Other(GenericBlockEntity)
}

impl BlockEntityTrait for BlockEntity {
    fn base(&self) -> &GenericBlockEntity {
        let this = self as &dyn BlockEntityTrait;
        this.base()
    }
}
