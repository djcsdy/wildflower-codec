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
