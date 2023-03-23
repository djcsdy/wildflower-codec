use crate::decode::tags::sounds::SoundEnvelopePoint;

#[derive(Clone, PartialEq, Debug)]
pub struct SoundInfo {
    pub sync_stop: bool,
    pub sync_no_multiple: bool,
    pub in_point: u32,
    pub out_point: u32,
    pub loop_count: u16,
    pub envelope_points: SoundEnvelopePoint,
}
