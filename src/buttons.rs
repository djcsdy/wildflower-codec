use super::actions::ActionRecord;
use super::common::{ColorTransformWithAlpha, Matrix};
use super::display_list::{BlendMode, Filter};

pub struct DefineButtonTag {
    pub button_id: u16,
    pub characters: Vec<ButtonRecord>,
    pub actions: Vec<ActionRecord>,
}

pub struct ButtonRecord {
    pub button_state_hit_test: bool,
    pub button_state_down: bool,
    pub button_state_over: bool,
    pub button_state_up: bool,
    pub character_id: u16,
    pub place_depth: u16,
    pub place_matrix: Matrix,
}

pub struct ButtonRecord2 {
    pub button_record: ButtonRecord,
    pub color_transform: ColorTransformWithAlpha,
    pub filter_list: Vec<Filter>,
    pub blend_mode: BlendMode,
}
