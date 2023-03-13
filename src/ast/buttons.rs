use super::actions::ActionRecord;
use super::common::*;
use super::display_list::{BlendMode, Filter};
use super::sounds::SoundInfo;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineButtonTag {
    pub button_id: u16,
    pub characters: Vec<ButtonRecord>,
    pub actions: Vec<ActionRecord>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DefineButton2Tag {
    pub button_id: u16,
    pub track_as_menu: bool,
    pub characters: Vec<ButtonRecord2>,
    pub actions: Vec<ButtonConditionActionRecord>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ButtonRecord {
    pub button_state_hit_test: bool,
    pub button_state_down: bool,
    pub button_state_over: bool,
    pub button_state_up: bool,
    pub character_id: u16,
    pub place_depth: u16,
    pub place_matrix: Matrix,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ButtonRecord2 {
    pub button_record: ButtonRecord,
    pub color_transform: ColorTransformWithAlpha,
    pub filter_list: Vec<Filter>,
    pub blend_mode: BlendMode,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ButtonConditionActionRecord {
    pub condition_idle_to_over_down: bool,
    pub condition_out_down_to_idle: bool,
    pub condition_out_down_to_over_down: bool,
    pub condition_over_down_to_out_down: bool,
    pub condition_over_down_to_over_up: bool,
    pub condition_over_up_to_over_down: bool,
    pub condition_over_up_to_idle: bool,
    pub condition_idle_to_over_up: bool,
    pub condition_key_press: u8,
    pub condition_over_down_to_idle: bool,
    pub actions: Vec<ActionRecord>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DefineButtonColorTransformTag {
    pub button_id: u16,
    pub button_color_transform: ColorTransform,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DefineButtonSoundTag {
    pub button_id: u16,
    pub button_sound_0: Option<ButtonSound>,
    pub button_sound_1: Option<ButtonSound>,
    pub button_sound_2: Option<ButtonSound>,
    pub button_sound_3: Option<ButtonSound>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ButtonSound {
    pub sound_id: u16,
    pub sound_info: SoundInfo,
}
