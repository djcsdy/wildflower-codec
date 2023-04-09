use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::sounds::sound_envelope_point::SoundEnvelopePoint;
use crate::decode::tags::sounds::sound_info_flags::SoundInfoFlags;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct SoundInfo {
    pub flags: SoundInfoFlags,
    pub in_point: u32,
    pub out_point: Option<u32>,
    pub loop_count: u16,
    pub envelope_points: Vec<SoundEnvelopePoint>,
}

impl SoundInfo {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let flags = SoundInfoFlags::read(reader)?;
        let in_point = if flags.contains(SoundInfoFlags::HAS_IN_POINT) {
            reader.read_u32()?
        } else {
            0
        };
        let out_point = if flags.contains(SoundInfoFlags::HAS_OUT_POINT) {
            Some(reader.read_u32()?)
        } else {
            None
        };
        let loop_count = if flags.contains(SoundInfoFlags::HAS_LOOPS) {
            reader.read_u16()?
        } else {
            1
        };
        let envelope_point_count = if flags.contains(SoundInfoFlags::HAS_ENVELOPE) {
            reader.read_u8()?
        } else {
            0
        };
        let mut envelope_points = Vec::with_capacity(envelope_point_count as usize);
        for _ in 0..envelope_point_count {
            envelope_points.push(SoundEnvelopePoint::read(reader)?);
        }
        Ok(Self {
            flags,
            in_point,
            out_point,
            loop_count,
            envelope_points,
        })
    }
}
