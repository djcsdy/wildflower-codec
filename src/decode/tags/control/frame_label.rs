use crate::decode::tags::common::string;

/// Labels the current frame with the specified name.
#[derive(Clone, PartialEq, Debug)]
pub struct FrameLabelTag {
    pub name: string::String,

    pub named_anchor: bool,
}
