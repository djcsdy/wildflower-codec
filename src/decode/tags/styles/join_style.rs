use crate::decode::tags::common::fixed_8::Fixed8;

#[derive(Clone, PartialEq, Debug)]
pub enum JoinStyle {
    Round,
    Bevel,
    Miter { miter_limit_factor: Fixed8 },
}
