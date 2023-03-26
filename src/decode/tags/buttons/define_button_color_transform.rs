use crate::decode::tags::common::color_transform::ColorTransform;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineButtonColorTransformTag {
    pub button_id: u16,
    pub button_color_transform: ColorTransform,
}
