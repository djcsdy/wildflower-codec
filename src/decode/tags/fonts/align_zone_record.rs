use crate::decode::tags::fonts::align_zone_data::AlignZoneData;

#[derive(Clone, PartialEq, Debug)]
pub struct AlignZoneRecord {
    pub zone_data: Vec<AlignZoneData>,
    pub zone_mask_y: bool,
    pub zone_mask_x: bool,
}
