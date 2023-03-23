use crate::decode::tags::buttons::button_record::ButtonRecord;
use crate::decode::tags::common::color_transform_with_alpha::ColorTransformWithAlpha;
use crate::decode::tags::display_list::{BlendMode, Filter};

#[derive(Clone, PartialEq, Debug)]
pub struct ButtonRecord2 {
    pub button_record: ButtonRecord,
    pub color_transform: ColorTransformWithAlpha,
    pub filter_list: Vec<Filter>,
    pub blend_mode: BlendMode,
}
