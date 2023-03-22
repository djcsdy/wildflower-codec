bitflags! {
    pub struct DefineEditTextFlags: u16 {
        const HAS_TEXT = 0x8000;
        const WORD_WRAP = 0x4000;
        const MULTILINE = 0x2000;
        const PASSWORD = 0x1000;
        const READ_ONLY = 0x0800;
        const HAS_TEXT_COLOR = 0x0400;
        const HAS_MAX_LENGTH = 0x0200;
        const HAS_FONT = 0x0100;
        const HAS_FONT_CLASS = 0x0080;
        const AUTOSIZE = 0x0040;
        const HAS_LAYOUT = 0x0020;
        const NO_SELECT = 0x0010;
        const BORDER = 0x0008;
        const WAS_STATIC = 0x0004;
        const HTML = 0x0002;
        const USE_OUTLINES = 0x0001;
    }
}
