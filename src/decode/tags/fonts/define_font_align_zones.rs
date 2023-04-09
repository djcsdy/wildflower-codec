use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::sized_read::SizedRead;
use crate::decode::tags::fonts::align_zone_record::AlignZoneRecord;
use crate::decode::tags::fonts::csm_table_hint::CsmTableHint;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFontAlignZonesTag {
    pub font_id: u16,
    pub csm_table_hint: CsmTableHint,
    pub zone_table: Vec<AlignZoneRecord>,
}

impl DefineFontAlignZonesTag {
    pub fn read<R: SizedRead + BitRead>(reader: &mut R) -> Result<Self> {
        let font_id = reader.read_u16()?;
        let csm_table_hint = CsmTableHint::read(reader)?;
        reader.read_ub8(6)?;
        let mut zone_table = Vec::new();
        while reader.remaining_bytes() > 0 {
            zone_table.push(AlignZoneRecord::read(reader)?);
        }
        Ok(Self {
            font_id,
            csm_table_hint,
            zone_table,
        })
    }
}
