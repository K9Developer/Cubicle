use crate::models::block_entity::block_entity::GenericBlockEntity;
use crate::models::other::inventory::Item;
use crate::traits::block_entity::BlockEntityTrait;

#[derive(Debug)]
pub struct LecternBlockEntity {
    base: GenericBlockEntity,
    book: Option<Item>,
    page: i32
}

impl LecternBlockEntity { // TODO: After customizing item then it should be Item::Book
    pub fn new(base: GenericBlockEntity, book: Option<Item>, page: i32) -> Self {
        LecternBlockEntity {
            base, book, page
        }
    }

    pub fn book(&self) -> &Option<Item> { &self.book }
    pub fn page(&self) -> i32 { self.page }

    pub fn set_selected_page(&mut self, page: i32) { self.page = page; }
    pub fn set_book(&mut self, book: Option<Item>) { self.book = book; }
}

impl BlockEntityTrait for LecternBlockEntity {
    fn base(&self) -> &GenericBlockEntity {
        &self.base
    }
    fn base_mut(&mut self) -> &mut GenericBlockEntity { &mut self.base }
}