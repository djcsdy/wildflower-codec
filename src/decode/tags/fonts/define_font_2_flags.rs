bitflags! {
    pub struct DefineFont2Flags: u8 {
        const HAS_LAYOUT = 0x80;
        const SHIFT_JIS = 0x40;
        const SMALL_TEXT = 0x20;
        const ANSI = 0x10;
        const WIDE_OFFSETS = 0x08;
        const WIDE_CODES = 0x04;
        const ITALIC = 0x02;
        const BOLD = 0x01;
    }
}
