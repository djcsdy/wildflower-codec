pub mod button_record;
pub mod button_record_2;
pub mod button_record_flags;

use super::actions::ActionRecord;
use crate::decode::tags::common::color_transform::ColorTransform;
use crate::decode::tags::sounds::sound_info::SoundInfo;
use button_record::ButtonRecord;
use button_record_2::ButtonRecord2;

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
