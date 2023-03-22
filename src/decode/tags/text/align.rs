use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Align {
    Left = 0,
    Right = 1,
    Center = 2,
    Justify = 3,
}
