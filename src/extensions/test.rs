use crate::models::world::world::World;

pub trait Test {
    fn draw(&self);
}

impl Test for World {
    fn draw(&self) {}
}

/*
draw will only be usable once this file is imported. This way means theres no ExtensionManager tho, so might need to find a better way.
*/