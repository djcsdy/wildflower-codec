pub mod button_condition;
pub mod button_condition_action_list;
pub mod button_condition_key_press;
pub mod button_record;
pub mod button_record_2;
pub mod button_record_flags;
pub mod define_button;
pub mod define_button_2;
pub mod define_button_2_flags;
pub mod define_button_color_transform;

use crate::decode::tags::sounds::sound_info::SoundInfo;

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
