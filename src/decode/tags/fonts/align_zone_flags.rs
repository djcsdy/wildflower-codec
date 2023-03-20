bitflags! {
    pub struct AlignZoneFlags: u8 {
        const RESERVED = 0xfc;
        const ZONE_MASK_Y = 0x02;
        const ZONE_MASK_X = 0x01;
    }
}
