use crate::decode::tags::fonts::zone_data::ZoneData;

#[derive(Clone, PartialEq, Debug)]
pub struct AlignZoneRecord {
    pub zone_data: Vec<ZoneData>,
    pub zone_mask_y: bool,
    pub zone_mask_x: bool,
}
