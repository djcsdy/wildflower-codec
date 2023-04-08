pub mod deblocking;
pub mod define_video_stream;

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Codec {
    SorensonH263 = 2,
    ScreenVideo = 3,
    Vp6 = 4,
    Vp6WithAlpha = 5,
}

#[derive(Clone, PartialEq, Debug)]
pub struct VideoFrameTag {
    pub stream_id: u16,
    pub frame_num: u16,
    pub video_data: Vec<u8>,
}
