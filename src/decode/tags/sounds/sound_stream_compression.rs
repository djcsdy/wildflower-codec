use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum SoundStreamCompression {
    Adpcm = 1,
    Mp3 = 2,
}
