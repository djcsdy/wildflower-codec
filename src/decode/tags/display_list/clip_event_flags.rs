bitflags! {
    pub struct ClipEventFlags: u32 {
        const KEY_UP = 0x8000_0000;
        const KEY_DOWN = 0x4000_0000;
        const MOUSE_UP = 0x2000_0000;
        const MOUSE_DOWN = 0x1000_0000;
        const MOUSE_MOVE = 0x0800_0000;
        const UNLOAD = 0x0400_0000;
        const ENTER_FRAME = 0x0200_0000;
        const LOAD = 0x0100_0000;
        const DRAG_OVER = 0x0080_0000;
        const ROLL_OUT = 0x0040_0000;
        const ROLL_OVER = 0x0020_0000;
        const RELEASE_OUTSIDE = 0x0010_0000;
        const RELEASE = 0x0008_0000;
        const PRESS = 0x0004_0000;
        const INITIALIZE = 0x0002_0000;
        const DATA = 0x0001_0000;
        const CONSTRUCT = 0x0000_0400;
        const KEY_PRESS = 0x0000_0200;
        const DRAG_OUT = 0x0000_0100;
        const RESERVED = 0x0000_f8ff;
    }
}
