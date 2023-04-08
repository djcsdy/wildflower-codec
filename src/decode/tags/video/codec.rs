use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Codec {
    SorensonH263 = 2,
    ScreenVideo = 3,
    Vp6 = 4,
    Vp6WithAlpha = 5,
}
