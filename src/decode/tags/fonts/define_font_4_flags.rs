bitflags! {
    pub struct DefineFont4Flags: u8 {
        const RESERVED = 0xf8;
        const HAS_FONT_DATA = 0x04;
        const ITALIC = 0x02;
        const BOLD = 0x01;
    }
}
