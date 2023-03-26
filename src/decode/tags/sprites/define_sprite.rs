use crate::decode::tags::sprites::ControlTag;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineSpriteTag {
    pub sprite_id: u16,
    pub frame_count: u16,
    pub control_tags: Vec<ControlTag>,
}
