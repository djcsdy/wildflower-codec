use crate::decode::tags::fonts::csm_table_hint::CsmTableHint;
use crate::decode::tags::fonts::AlignZoneRecord;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFontAlignZonesTag {
    pub font_id: u16,
    pub csm_table_hint: CsmTableHint,
    pub zones: Vec<AlignZoneRecord>,
}
