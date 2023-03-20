use crate::decode::tags::fonts::align_zone_record::AlignZoneRecord;
use crate::decode::tags::fonts::csm_table_hint::CsmTableHint;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFontAlignZonesTag {
    pub font_id: u16,
    pub csm_table_hint: CsmTableHint,
    pub zone_table: Vec<AlignZoneRecord>,
}
