use crate::decode::tags::video::deblocking::Deblocking;
use crate::decode::tags::video::Codec;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineVideoStreamTag {
    pub character_id: u16,
    pub num_frames: u16,
    pub width: u16,
    pub height: u16,
    pub deblocking: Deblocking,
    pub smoothing: bool,
    pub codec: Codec,
}
