pub mod define_video_stream;

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Deblocking {
    VideoPacket = 0b000,
    Off = 0b001,
    Level1 = 0b010,
    Level2 = 0b011,
    Level3 = 0b100,
    Level4 = 0b101,
}

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
