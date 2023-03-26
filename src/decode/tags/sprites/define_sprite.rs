use crate::decode::tags::sprites::control_tag_list::ControlTagList;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineSpriteTag {
    pub sprite_id: u16,
    pub frame_count: u16,
    pub control_tags: ControlTagList<Vec<u8>>
}
