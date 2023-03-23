use crate::decode::tags::sounds::sound_envelope_point::SoundEnvelopePoint;
use crate::decode::tags::sounds::sound_info_flags::SoundInfoFlags;

#[derive(Clone, PartialEq, Debug)]
pub struct SoundInfo {
    pub flags: SoundInfoFlags,
    pub in_point: u32,
    pub out_point: u32,
    pub loop_count: u16,
    pub envelope_points: SoundEnvelopePoint,
}
