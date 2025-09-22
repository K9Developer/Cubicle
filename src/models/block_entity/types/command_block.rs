use crate::models::block_entity::block_entity::GenericBlockEntity;
use crate::models::other::mojang_data::text_component::TextComponent;
use crate::models::other::tick::Tick;

pub struct CommandBlockBlockEntity {
    base: GenericBlockEntity,

    is_auto: bool,
    command: String, // TODO: This will become MinecraftCommand to translate and interact
    is_condition_met: bool,
    custom_name: Option<TextComponent>,
    last_executed: Tick,
    last_output: String,
    is_powered: bool,
    success_count: i32,
    is_keep_last_output: bool,
    is_update_last_executed: bool,
}

impl CommandBlockBlockEntity {
    pub fn new(
        base: GenericBlockEntity,
        is_auto: bool,
        command: String,
        is_condition_met: bool,
        custom_name: Option<TextComponent>,
        last_executed: Tick,
        last_output: String,
        is_powered: bool,
        success_count: i32,
        is_keep_last_output: bool,
        is_update_last_executed: bool,
    ) -> Self {
        CommandBlockBlockEntity {
            base, is_auto, command, is_condition_met, custom_name, last_executed, last_output, is_powered, success_count, is_keep_last_output, is_update_last_executed,
        }
    }

    pub fn is_auto(&self) -> bool { self.is_auto }
    pub fn is_condition_met(&self) -> bool { self.is_condition_met }
    pub fn custom_name(&self) -> Option<&TextComponent> { self.custom_name.as_ref() }
    pub fn custom_name_mut(&mut self) -> Option<&mut TextComponent> { self.custom_name.as_mut() }
    pub fn command(&self) -> &String { &self.command }
    pub fn last_executed(&self) -> &Tick { &self.last_executed }
    pub fn last_output(&self) -> &String { &self.last_output }
    pub fn is_powered(&self) -> bool { self.is_powered }
    pub fn success_count(&self) -> i32 { self.success_count }
    pub fn is_keep_last_output(&self) -> bool { self.is_keep_last_output }
    pub fn is_update_last_executed(&self) -> bool { self.is_update_last_executed }

    pub fn set_is_auto(&mut self, is_auto: bool) { self.is_auto = is_auto; }
    pub fn set_is_condition_met(&mut self, is_condition_met: bool) { self.is_condition_met = is_condition_met; }
    pub fn set_custom_name(&mut self, custom_name: Option<TextComponent>) { self.custom_name = custom_name; }
    pub fn set_command(&mut self, command: String) { self.command = command; }
    pub fn set_last_executed(&mut self, tick: Tick) { self.last_executed = tick; }
    pub fn set_last_output(&mut self, output: String) { self.last_output = output; }
    pub fn set_is_powered(&mut self, is_pow: bool) { self.is_powered = is_pow; }
    pub fn set_success_count(&mut self, success_count: i32) { self.success_count = success_count; }
    pub fn set_is_keep_last_output(&mut self, is_keep_last_output: bool) { self.is_keep_last_output = is_keep_last_output; }
    pub fn set_is_update_last_executed(&mut self, is_update_last_executed: bool) { self.is_update_last_executed = is_update_last_executed; }
}