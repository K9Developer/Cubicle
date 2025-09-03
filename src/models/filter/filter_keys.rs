// If adding keys here, also add key to Filter::key_to_block_key, etc.
pub mod FilterKey { // TODO: Expand on this...
    pub const X_POSITION: &'static str = "key:x";
    pub const Y_POSITION: &'static str = "key:y";
    pub const Z_POSITION: &'static str = "key:z";

    pub const POSITION: &'static str = "key:pos";
    pub const ID: &'static str = "key:id";
}