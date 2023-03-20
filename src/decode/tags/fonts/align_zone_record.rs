use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::fonts::align_zone_data::AlignZoneData;
use crate::decode::tags::fonts::align_zone_flags::AlignZoneFlags;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct AlignZoneRecord {
    pub zone_data: Vec<AlignZoneData>,
    pub flags: AlignZoneFlags,
}

impl AlignZoneRecord {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let num_zone_data = reader.read_u8()?;
        let mut zone_data = Vec::with_capacity(num_zone_data as usize);
        for _ in 0..num_zone_data {
            zone_data.push(AlignZoneData::read(reader)?);
        }
        let flags = AlignZoneFlags::read(reader)?;
        Ok(Self { zone_data, flags })
    }
}
