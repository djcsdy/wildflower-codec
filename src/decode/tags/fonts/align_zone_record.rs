use crate::decode::tags::fonts::align_zone_data::AlignZoneData;
use crate::decode::tags::fonts::align_zone_flags::AlignZoneFlags;

#[derive(Clone, PartialEq, Debug)]
pub struct AlignZoneRecord {
    pub zone_data: Vec<AlignZoneData>,
    pub flags: AlignZoneFlags,
}
