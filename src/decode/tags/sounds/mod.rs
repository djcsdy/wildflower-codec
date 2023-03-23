pub mod define_sound;
pub mod sampling_bits;
pub mod sampling_rate;
pub mod sound_channels;
pub mod sound_envelope_point;
pub mod sound_format;
pub mod sound_info;
pub mod sound_info_flags;
pub mod sound_stream_compression;
pub mod sound_stream_head;
pub mod start_sound;
pub mod start_sound_2;

use sampling_bits::SamplingBits;
use sampling_rate::SamplingRate;
use sound_channels::SoundChannels;
use sound_format::SoundFormat;

#[derive(Clone, PartialEq, Debug)]
pub struct SoundStreamHead2Tag {
    pub playback_sound_rate: SamplingRate,
    pub playback_channels: SoundChannels,
    pub stream_sound_compression: SoundFormat,
    pub stream_sound_rate: SamplingRate,
    pub stream_sound_bits: SamplingBits,
    pub stream_sound_channels: SoundChannels,
    pub stream_sound_sample_count: u16,
    pub latency_seek: i16,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SoundStreamBlockTag {
    pub stream_sound_data: Vec<u8>,
}
