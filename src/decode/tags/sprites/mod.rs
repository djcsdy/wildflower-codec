use super::control::{EndTag, FrameLabelTag};
use super::display_list::{
    PlaceObject2Tag, PlaceObjectTag, RemoveObject2Tag, RemoveObjectTag, ShowFrameTag,
};
use crate::decode::tags::actions::do_abc::DoAbcTag;
use crate::decode::tags::actions::do_action::DoActionTag;
use crate::decode::tags::actions::do_init_action::DoInitActionTag;
use crate::decode::tags::sounds::sound_stream_block::SoundStreamBlockTag;
use crate::decode::tags::sounds::sound_stream_head::SoundStreamHeadTag;
use crate::decode::tags::sounds::sound_stream_head_2::SoundStreamHead2Tag;
use crate::decode::tags::sounds::start_sound::StartSoundTag;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineSpriteTag {
    pub sprite_id: u16,
    pub frame_count: u16,
    pub control_tags: Vec<ControlTag>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ControlTag {
    ShowFrame(ShowFrameTag),
    PlaceObject(PlaceObjectTag),
    PlaceObject2(PlaceObject2Tag),
    RemoveObject(RemoveObjectTag),
    RemoveObject2(RemoveObject2Tag),
    DoAbc(DoAbcTag),
    DoAction(DoActionTag),
    DoInitAction(DoInitActionTag),
    StartSound(StartSoundTag),
    FrameLabel(FrameLabelTag),
    SoundStreamHead(SoundStreamHeadTag),
    SoundStreamHead2(SoundStreamHead2Tag),
    SoundStreamBlock(SoundStreamBlockTag),
    End(EndTag),
}
