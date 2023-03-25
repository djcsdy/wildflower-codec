use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum ButtonConditionKeyPress {
    LeftArrow = 1,
    RightArrow = 2,
    Home = 3,
    End = 4,
    Insert = 5,
    Delete = 6,
    Backspace = 8,
    Enter = 13,
    UpArrow = 14,
    DownArrow = 15,
    PageUp = 16,
    PageDown = 17,
    Tab = 18,
    Escape = 19,
    #[num_enum(catch_all)]
    Ascii(u8),
}
