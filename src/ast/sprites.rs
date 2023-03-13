use super::actions::{DoAbcTag, DoActionTag, DoInitActionTag};
use super::control::{EndTag, FrameLabelTag};
use super::display_list::{
    PlaceObject2Tag, PlaceObjectTag, RemoveObject2Tag, RemoveObjectTag, ShowFrameTag,
};
use super::sounds::{SoundStreamBlockTag, SoundStreamHead2Tag, SoundStreamHeadTag, StartSoundTag};

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
