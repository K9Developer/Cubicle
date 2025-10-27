use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use fastnbt::Value;
use crate::models::other::properties::Properties;
use crate::ValueConversion;
use super::block_state_types::*;

// https://minecraft.wiki/w/Block_states#List_of_block_states
pub struct BlockStates {
    props: Properties,
    raw_map: Arc<Mutex<HashMap<String, Value>>>,
}

impl BlockStates {
    pub fn new(props: Properties) -> Self {
        BlockStates { raw_map: props.properties_raw(), props }
    }
}

// getters
impl BlockStates {
    fn p(&self) -> MutexGuard<HashMap<String, Value>> { self.raw_map.lock().unwrap() }

    pub fn age(&self) -> Option<i32> { self.p().get("age")?.as_i32().cloned() }
    pub fn set_age(&mut self, value: i32) -> &mut Self { self.p().insert("age".to_string(), Value::Int(value)); self }

    pub fn is_attached(&self) -> Option<bool> { self.p().get("attached")?.as_bool() }
    pub fn set_is_attached(&mut self, value: bool) -> &mut Self { self.p().insert("attached".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn attachment(&self) -> Option<Attachment> { Attachment::from_nbt(self.p().get("attachment")) }
    pub fn set_attachment(&mut self, value: Attachment) -> &mut Self { self.p().insert("attachment".to_string(), value.to_nbt()); self }

    pub fn axis(&self) -> Option<Axis> { Axis::from_nbt(self.p().get("axis")) }
    pub fn set_axis(&mut self, value: Axis) -> &mut Self { self.p().insert("axis".to_string(), value.to_nbt()); self }

    pub fn has_berries(&self) -> Option<bool> { self.p().get("berries")?.as_bool() }
    pub fn set_has_berries(&mut self, value: bool) -> &mut Self { self.p().insert("berries".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn cake_bites_taken(&self) -> Option<i32> { self.p().get("bites")?.as_i32().cloned() }
    pub fn set_cake_bites_taken(&mut self, value: i32) -> &mut Self { self.p().insert("bites".to_string(), Value::Int(value)); self }

    pub fn is_sculk_catalyst_blooming(&self) -> Option<bool> { self.p().get("bloom")?.as_bool() }
    pub fn set_is_sculk_catalyst_blooming(&mut self, value: bool) -> &mut Self { self.p().insert("bloom".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn is_scaffolding_bottom(&self) -> Option<bool> { self.p().get("bottom")?.as_bool() }
    pub fn set_is_scaffolding_bottom(&mut self, value: bool) -> &mut Self { self.p().insert("bottom".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn can_sculk_shrieker_summon(&self) -> Option<bool> { self.p().get("can_summon")?.as_bool() }
    pub fn set_can_sculk_shrieker_summon(&mut self, value: bool) -> &mut Self { self.p().insert("can_summon".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn candle_count(&self) -> Option<i32> { self.p().get("candles")?.as_i32().cloned() }
    pub fn set_candle_count(&mut self, value: i32) -> &mut Self { self.p().insert("candles".to_string(), Value::Int(value)); self }

    pub fn respawn_anchor_charges(&self) -> Option<i32> { self.p().get("charges")?.as_i32().cloned() }
    pub fn set_respawn_anchor_charges(&mut self, value: i32) -> &mut Self { self.p().insert("charges".to_string(), Value::Int(value)); self }

    pub fn is_command_block_conditional(&self) -> Option<bool> { self.p().get("conditional")?.as_bool() }
    pub fn set_is_command_block_conditional(&mut self, value: bool) -> &mut Self { self.p().insert("conditional".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn copper_golem_pose(&self) -> Option<CopperGolemPose> { CopperGolemPose::from_nbt(self.p().get("copper_golem_pose")) }
    pub fn set_copper_golem_pose(&mut self, value: CopperGolemPose) -> &mut Self { self.p().insert("copper_golem_pose".to_string(), value.to_nbt()); self }

    pub fn is_decorated_pot_cracked(&self) -> Option<bool> { self.p().get("cracked")?.as_bool() }
    pub fn set_is_decorated_pot_cracked(&mut self, value: bool) -> &mut Self { self.p().insert("cracked".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn is_crafter_crafting(&self) -> Option<bool> { self.p().get("crafting")?.as_bool() }
    pub fn set_is_crafter_crafting(&mut self, value: bool) -> &mut Self { self.p().insert("crafting".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn creaking_heart_state(&self) -> Option<CreakingHeartState> { CreakingHeartState::from_nbt(self.p().get("creaking_heart_state")) }
    pub fn set_creaking_heart_state(&mut self, value: CreakingHeartState) -> &mut Self { self.p().insert("creaking_heart_state".to_string(), value.to_nbt()); self }

    pub fn repeater_delay(&self) -> Option<i32> { self.p().get("delay")?.as_i32().cloned() }
    pub fn set_repeater_delay(&mut self, value: i32) -> &mut Self { self.p().insert("delay".to_string(), Value::Int(value)); self }

    pub fn is_tripwire_disarmed(&self) -> Option<bool> { self.p().get("disarmed")?.as_bool() }
    pub fn set_is_tripwire_disarmed(&mut self, value: bool) -> &mut Self { self.p().insert("disarmed".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn distance_from_log(&self) -> Option<i32> { self.p().get("distance")?.as_i32().cloned() }
    pub fn set_distance_from_log(&mut self, value: i32) -> &mut Self { self.p().insert("distance".to_string(), Value::Int(value)); self }

    pub fn connects_down(&self) -> Option<bool> { self.p().get("down")?.as_bool() }
    pub fn set_connects_down(&mut self, value: bool) -> &mut Self { self.p().insert("down".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn is_bubble_column_drag(&self) -> Option<bool> { self.p().get("drag")?.as_bool() }
    pub fn set_is_bubble_column_drag(&mut self, value: bool) -> &mut Self { self.p().insert("drag".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn suspicious_block_dusted(&self) -> Option<i32> { self.p().get("dusted")?.as_i32().cloned() }
    pub fn set_suspicious_block_dusted(&mut self, value: i32) -> &mut Self { self.p().insert("dusted".to_string(), Value::Int(value)); self }

    pub fn east_wall_connection(&self) -> Option<WallConnection> { WallConnection::from_nbt(self.p().get("east")) }
    pub fn set_east_wall_connection(&mut self, value: WallConnection) -> &mut Self { self.p().insert("east".to_string(), value.to_nbt()); self }

    pub fn turtle_egg_count(&self) -> Option<i32> { self.p().get("eggs")?.as_i32().cloned() }
    pub fn set_turtle_egg_count(&mut self, value: i32) -> &mut Self { self.p().insert("eggs".to_string(), Value::Int(value)); self }

    pub fn is_hopper_enabled(&self) -> Option<bool> { self.p().get("enabled")?.as_bool() }
    pub fn set_is_hopper_enabled(&mut self, value: bool) -> &mut Self { self.p().insert("enabled".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn is_piston_extended(&self) -> Option<bool> { self.p().get("extended")?.as_bool() }
    pub fn set_is_piston_extended(&mut self, value: bool) -> &mut Self { self.p().insert("extended".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn has_end_portal_frame_eye(&self) -> Option<bool> { self.p().get("eye")?.as_bool() }
    pub fn set_has_end_portal_frame_eye(&mut self, value: bool) -> &mut Self { self.p().insert("eye".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn attachment_face(&self) -> Option<Face> { Face::from_nbt(self.p().get("face")) }
    pub fn set_attachment_face(&mut self, value: Face) -> &mut Self { self.p().insert("face".to_string(), value.to_nbt()); self }

    pub fn facing(&self) -> Option<Direction> { Direction::from_nbt(self.p().get("facing")) }
    pub fn set_facing(&mut self, value: Direction) -> &mut Self { self.p().insert("facing".to_string(), value.to_nbt()); self }

    pub fn is_fluid_falling(&self) -> Option<bool> { self.p().get("falling")?.as_bool() }
    pub fn set_is_fluid_falling(&mut self, value: bool) -> &mut Self { self.p().insert("falling".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn flower_amount(&self) -> Option<i32> { self.p().get("flower_amount")?.as_i32().cloned() }
    pub fn set_flower_amount(&mut self, value: i32) -> &mut Self { self.p().insert("flower_amount".to_string(), Value::Int(value)); self }

    pub fn half(&self) -> Option<Half> { Half::from_nbt(self.p().get("half")) }
    pub fn set_half(&mut self, value: Half) -> &mut Self { self.p().insert("half".to_string(), value.to_nbt()); self }

    pub fn is_lantern_hanging(&self) -> Option<bool> { self.p().get("hanging")?.as_bool() }
    pub fn set_is_lantern_hanging(&mut self, value: bool) -> &mut Self { self.p().insert("hanging".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn lectern_has_book(&self) -> Option<bool> { self.p().get("has_book")?.as_bool() }
    pub fn set_lectern_has_book(&mut self, value: bool) -> &mut Self { self.p().insert("has_book".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn brewing_stand_has_bottle_0(&self) -> Option<bool> { self.p().get("has_bottle_0")?.as_bool() }
    pub fn set_brewing_stand_has_bottle_0(&mut self, value: bool) -> &mut Self { self.p().insert("has_bottle_0".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn brewing_stand_has_bottle_1(&self) -> Option<bool> { self.p().get("has_bottle_1")?.as_bool() }
    pub fn set_brewing_stand_has_bottle_1(&mut self, value: bool) -> &mut Self { self.p().insert("has_bottle_1".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn brewing_stand_has_bottle_2(&self) -> Option<bool> { self.p().get("has_bottle_2")?.as_bool() }
    pub fn set_brewing_stand_has_bottle_2(&mut self, value: bool) -> &mut Self { self.p().insert("has_bottle_2".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn jukebox_has_record(&self) -> Option<bool> { self.p().get("has_record")?.as_bool() }
    pub fn set_jukebox_has_record(&mut self, value: bool) -> &mut Self { self.p().insert("has_record".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn egg_hatch_stage(&self) -> Option<i32> { self.p().get("hatch")?.as_i32().cloned() }
    pub fn set_egg_hatch_stage(&mut self, value: i32) -> &mut Self { self.p().insert("hatch".to_string(), Value::Int(value)); self }

    pub fn door_hinge(&self) -> Option<Hinge> { Hinge::from_nbt(self.p().get("hinge")) }
    pub fn set_door_hinge(&mut self, value: Hinge) -> &mut Self { self.p().insert("hinge".to_string(), value.to_nbt()); self }

    pub fn beehive_honey_level(&self) -> Option<i32> { self.p().get("honey_level")?.as_i32().cloned() }
    pub fn set_beehive_honey_level(&mut self, value: i32) -> &mut Self { self.p().insert("honey_level".to_string(), Value::Int(value)); self }

    pub fn is_fence_gate_in_wall(&self) -> Option<bool> { self.p().get("in_wall")?.as_bool() }
    pub fn set_is_fence_gate_in_wall(&mut self, value: bool) -> &mut Self { self.p().insert("in_wall".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn note_block_instrument(&self) -> Option<Instrument> { Instrument::from_nbt(self.p().get("instrument")) }
    pub fn set_note_block_instrument(&mut self, value: Instrument) -> &mut Self { self.p().insert("instrument".to_string(), value.to_nbt()); self }

    pub fn is_daylight_detector_inverted(&self) -> Option<bool> { self.p().get("inverted")?.as_bool() }
    pub fn set_is_daylight_detector_inverted(&mut self, value: bool) -> &mut Self { self.p().insert("inverted".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn snow_layers(&self) -> Option<i32> { self.p().get("layers")?.as_i32().cloned() }
    pub fn set_snow_layers(&mut self, value: i32) -> &mut Self { self.p().insert("layers".to_string(), Value::Int(value)); self }

    pub fn bamboo_leaves(&self) -> Option<Leaves> { Leaves::from_nbt(self.p().get("leaves")) }
    pub fn set_bamboo_leaves(&mut self, value: Leaves) -> &mut Self { self.p().insert("leaves".to_string(), value.to_nbt()); self }

    pub fn level(&self) -> Option<i32> { self.p().get("level")?.as_i32().cloned() }
    pub fn set_level(&mut self, value: i32) -> &mut Self { self.p().insert("level".to_string(), Value::Int(value)); self }

    pub fn is_lit(&self) -> Option<bool> { self.p().get("lit")?.as_bool() }
    pub fn set_is_lit(&mut self, value: bool) -> &mut Self { self.p().insert("lit".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn is_repeater_locked(&self) -> Option<bool> { self.p().get("locked")?.as_bool() }
    pub fn set_is_repeater_locked(&mut self, value: bool) -> &mut Self { self.p().insert("locked".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn mode(&self) -> Option<Mode> { Mode::from_nbt(self.p().get("mode")) }
    pub fn set_mode(&mut self, value: Mode) -> &mut Self { self.p().insert("mode".to_string(), value.to_nbt()); self }

    pub fn farmland_moisture(&self) -> Option<i32> { self.p().get("moisture")?.as_i32().cloned() }
    pub fn set_farmland_moisture(&mut self, value: i32) -> &mut Self { self.p().insert("moisture".to_string(), Value::Int(value)); self }

    pub fn is_creaking_heart_natural(&self) -> Option<bool> { self.p().get("natural")?.as_bool() }
    pub fn set_is_creaking_heart_natural(&mut self, value: bool) -> &mut Self { self.p().insert("natural".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn north_wall_connection(&self) -> Option<WallConnection> { WallConnection::from_nbt(self.p().get("north")) }
    pub fn set_north_wall_connection(&mut self, value: WallConnection) -> &mut Self { self.p().insert("north".to_string(), value.to_nbt()); self }

    pub fn note_block_note(&self) -> Option<i32> { self.p().get("note")?.as_i32().cloned() }
    pub fn set_note_block_note(&mut self, value: i32) -> &mut Self { self.p().insert("note".to_string(), Value::Int(value)); self }

    pub fn is_bed_occupied(&self) -> Option<bool> { self.p().get("occupied")?.as_bool() }
    pub fn set_is_bed_occupied(&mut self, value: bool) -> &mut Self { self.p().insert("occupied".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn is_ominous(&self) -> Option<bool> { self.p().get("ominous")?.as_bool() }
    pub fn set_is_ominous(&mut self, value: bool) -> &mut Self { self.p().insert("ominous".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn is_door_open(&self) -> Option<bool> { self.p().get("open")?.as_bool() }
    pub fn set_is_door_open(&mut self, value: bool) -> &mut Self { self.p().insert("open".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn jigsaw_orientation(&self) -> Option<Orientation> { Orientation::from_nbt(self.p().get("orientation")) }
    pub fn set_jigsaw_orientation(&mut self, value: Orientation) -> &mut Self { self.p().insert("orientation".to_string(), value.to_nbt()); self }

    pub fn bed_part(&self) -> Option<Part> { Part::from_nbt(self.p().get("part")) }
    pub fn set_bed_part(&mut self, value: Part) -> &mut Self { self.p().insert("part".to_string(), value.to_nbt()); self }

    pub fn are_leaves_persistent(&self) -> Option<bool> { self.p().get("persistent")?.as_bool() }
    pub fn set_are_leaves_persistent(&mut self, value: bool) -> &mut Self { self.p().insert("persistent".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn pickle_count(&self) -> Option<i32> { self.p().get("pickles")?.as_i32().cloned() }
    pub fn set_pickle_count(&mut self, value: i32) -> &mut Self { self.p().insert("pickles".to_string(), Value::Int(value)); self }

    pub fn redstone_power(&self) -> Option<i32> { self.p().get("power")?.as_i32().cloned() }
    pub fn set_redstone_power(&mut self, value: i32) -> &mut Self { self.p().insert("power".to_string(), Value::Int(value)); self }

    pub fn is_powered(&self) -> Option<bool> { self.p().get("powered")?.as_bool() }
    pub fn set_is_powered(&mut self, value: bool) -> &mut Self { self.p().insert("powered".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn rotation(&self) -> Option<i32> { self.p().get("rotation")?.as_i32().cloned() }
    pub fn set_rotation(&mut self, value: i32) -> &mut Self { self.p().insert("rotation".to_string(), Value::Int(value)); self }

    pub fn sculk_sensor_phase(&self) -> Option<SculkSensorPhase> { SculkSensorPhase::from_nbt(self.p().get("sculk_sensor_phase")) }
    pub fn set_sculk_sensor_phase(&mut self, value: SculkSensorPhase) -> &mut Self { self.p().insert("sculk_sensor_phase".to_string(), value.to_nbt()); self }

    pub fn leaf_litter_segment_amount(&self) -> Option<i32> { self.p().get("segment_amount")?.as_i32().cloned() }
    pub fn set_leaf_litter_segment_amount(&mut self, value: i32) -> &mut Self { self.p().insert("segment_amount".to_string(), Value::Int(value)); self }

    pub fn rail_shape(&self) -> Option<Shape> { Shape::from_nbt(self.p().get("shape")) }
    pub fn set_rail_shape(&mut self, value: Shape) -> &mut Self { self.p().insert("shape".to_string(), value.to_nbt()); self }

    pub fn is_piston_head_short(&self) -> Option<bool> { self.p().get("short")?.as_bool() }
    pub fn set_is_piston_head_short(&mut self, value: bool) -> &mut Self { self.p().insert("short".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn is_sculk_shrieker_shrieking(&self) -> Option<bool> { self.p().get("shrieking")?.as_bool() }
    pub fn set_is_sculk_shrieker_shrieking(&mut self, value: bool) -> &mut Self { self.p().insert("shrieking".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn vault_side_chain(&self) -> Option<VaultState> { VaultState::from_nbt(self.p().get("side_chain")) }
    pub fn set_vault_side_chain(&mut self, value: VaultState) -> &mut Self { self.p().insert("side_chain".to_string(), value.to_nbt()); self }

    pub fn is_campfire_signal_fire(&self) -> Option<bool> { self.p().get("signal_fire")?.as_bool() }
    pub fn set_is_campfire_signal_fire(&mut self, value: bool) -> &mut Self { self.p().insert("signal_fire".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn is_chiseled_bookshelf_slot_0_occupied(&self) -> Option<bool> { self.p().get("slot_0_occupied")?.as_bool() }
    pub fn set_is_chiseled_bookshelf_slot_0_occupied(&mut self, value: bool) -> &mut Self { self.p().insert("slot_0_occupied".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn is_chiseled_bookshelf_slot_1_occupied(&self) -> Option<bool> { self.p().get("slot_1_occupied")?.as_bool() }
    pub fn set_is_chiseled_bookshelf_slot_1_occupied(&mut self, value: bool) -> &mut Self { self.p().insert("slot_1_occupied".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn is_chiseled_bookshelf_slot_2_occupied(&self) -> Option<bool> { self.p().get("slot_2_occupied")?.as_bool() }
    pub fn set_is_chiseled_bookshelf_slot_2_occupied(&mut self, value: bool) -> &mut Self { self.p().insert("slot_2_occupied".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn is_chiseled_bookshelf_slot_3_occupied(&self) -> Option<bool> { self.p().get("slot_3_occupied")?.as_bool() }
    pub fn set_is_chiseled_bookshelf_slot_3_occupied(&mut self, value: bool) -> &mut Self { self.p().insert("slot_3_occupied".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn is_chiseled_bookshelf_slot_4_occupied(&self) -> Option<bool> { self.p().get("slot_4_occupied")?.as_bool() }
    pub fn set_is_chiseled_bookshelf_slot_4_occupied(&mut self, value: bool) -> &mut Self { self.p().insert("slot_4_occupied".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn is_chiseled_bookshelf_slot_5_occupied(&self) -> Option<bool> { self.p().get("slot_5_occupied")?.as_bool() }
    pub fn set_is_chiseled_bookshelf_slot_5_occupied(&mut self, value: bool) -> &mut Self { self.p().insert("slot_5_occupied".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn has_snowy_texture(&self) -> Option<bool> { self.p().get("snowy")?.as_bool() }
    pub fn set_has_snowy_texture(&mut self, value: bool) -> &mut Self { self.p().insert("snowy".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn south_wall_connection(&self) -> Option<WallConnection> { WallConnection::from_nbt(self.p().get("south")) }
    pub fn set_south_wall_connection(&mut self, value: WallConnection) -> &mut Self { self.p().insert("south".to_string(), value.to_nbt()); self }

    pub fn sapling_stage(&self) -> Option<i32> { self.p().get("stage")?.as_i32().cloned() }
    pub fn set_sapling_stage(&mut self, value: i32) -> &mut Self { self.p().insert("stage".to_string(), Value::Int(value)); self }

    pub fn dripstone_thickness(&self) -> Option<Thickness> { Thickness::from_nbt(self.p().get("thickness")) }
    pub fn set_dripstone_thickness(&mut self, value: Thickness) -> &mut Self { self.p().insert("thickness".to_string(), value.to_nbt()); self }

    pub fn big_dripleaf_tilt(&self) -> Option<Tilt> { Tilt::from_nbt(self.p().get("tilt")) }
    pub fn set_big_dripleaf_tilt(&mut self, value: Tilt) -> &mut Self { self.p().insert("tilt".to_string(), value.to_nbt()); self }

    pub fn is_pale_hanging_moss_tip(&self) -> Option<bool> { self.p().get("tip")?.as_bool() }
    pub fn set_is_pale_hanging_moss_tip(&mut self, value: bool) -> &mut Self { self.p().insert("tip".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn trial_spawner_state(&self) -> Option<TrialSpawnerState> { TrialSpawnerState::from_nbt(self.p().get("trial_spawner_state")) }
    pub fn set_trial_spawner_state(&mut self, value: TrialSpawnerState) -> &mut Self { self.p().insert("trial_spawner_state".to_string(), value.to_nbt()); self }

    pub fn is_triggered(&self) -> Option<bool> { self.p().get("triggered")?.as_bool() }
    pub fn set_is_triggered(&mut self, value: bool) -> &mut Self { self.p().insert("triggered".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn block_type(&self) -> Option<BlockType> { BlockType::from_nbt(self.p().get("type")) }
    pub fn set_block_type(&mut self, value: BlockType) -> &mut Self { self.p().insert("type".to_string(), value.to_nbt()); self }

    pub fn is_tnt_unstable(&self) -> Option<bool> { self.p().get("unstable")?.as_bool() }
    pub fn set_is_tnt_unstable(&mut self, value: bool) -> &mut Self { self.p().insert("unstable".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn connects_up(&self) -> Option<bool> { self.p().get("up")?.as_bool() }
    pub fn set_connects_up(&mut self, value: bool) -> &mut Self { self.p().insert("up".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn vault_state(&self) -> Option<VaultState> { VaultState::from_nbt(self.p().get("vault_state")) }
    pub fn set_vault_state(&mut self, value: VaultState) -> &mut Self { self.p().insert("vault_state".to_string(), value.to_nbt()); self }

    pub fn dripstone_vertical_direction(&self) -> Option<VerticalDirection> { VerticalDirection::from_nbt(self.p().get("vertical_direction")) }
    pub fn set_dripstone_vertical_direction(&mut self, value: VerticalDirection) -> &mut Self { self.p().insert("vertical_direction".to_string(), value.to_nbt()); self }

    pub fn is_waterlogged(&self) -> Option<bool> { self.p().get("waterlogged")?.as_bool() }
    pub fn set_is_waterlogged(&mut self, value: bool) -> &mut Self { self.p().insert("waterlogged".to_string(), Value::Byte(if value {1} else {0})); self }

    pub fn west_wall_connection(&self) -> Option<WallConnection> { WallConnection::from_nbt(self.p().get("west")) }
    pub fn set_west_wall_connection(&mut self, value: WallConnection) -> &mut Self { self.p().insert("west".to_string(), value.to_nbt()); self }

    pub fn all(&self) -> &Properties { &self.props }
    pub fn all_mut(&mut self) -> &mut Properties { &mut self.props }
}




/*
// TODO: https://minecraft.wiki/w/Block_states shows what blocks can have this block state, maybe add some getter for it or have info for it
// TODO: version appropriate (self.version.has_berries_bs_key)
// TODO: In the fututure make this code auto generated with build using the jar data and some naming rules
*/