use crate::decode::tags::common::rectangle::Rectangle;

/// Defines a 9-slice grid that should be applied when scaling the specified
/// character.
#[derive(Clone, PartialEq, Debug)]
pub struct DefineScalingGridTag {
    pub character_id: u16,
    pub splitter: Rectangle,
}
