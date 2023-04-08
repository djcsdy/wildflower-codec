bitflags! {
    pub struct FileAttributesFlags: u32 {
        const USE_DIRECT_BLIT = 0x4000_0000;
        const USE_GPU = 0x2000_0000;
        const HAS_METADATA = 0x1000_0000;
        const ACTION_SCRIPT_3 = 0x0800_0000;
        const USE_NETWORK = 0x0100_0000;
        const RESERVED = 0x86ff_ffff;
    }
}
