use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub(crate) enum BitmapFormat {
    ColorMap8 = 3,
    Rgb15 = 4,
    Rgb24 = 5,
}
