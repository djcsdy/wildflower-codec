use crate::decode::tags::sounds::sampling_rate::SamplingRate;
use crate::decode::tags::sounds::sound_channels::SoundChannels;
use crate::decode::tags::sounds::SoundStreamCompression;

#[derive(Clone, PartialEq, Debug)]
pub struct SoundStreamHeadTag {
    pub playback_sound_rate: SamplingRate,
    pub playback_channels: SoundChannels,
    pub stream_sound_compression: SoundStreamCompression,
    pub stream_sound_rate: SamplingRate,
    pub stream_sound_channels: SoundChannels,
    pub stream_sound_sample_count: u16,
    pub latency_seek: i16,
}
