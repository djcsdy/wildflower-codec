pub mod define_sound;
pub mod sampling_bits;
pub mod sampling_rate;
pub mod sound_channels;
pub mod sound_format;
pub mod sound_info;
pub mod sound_info_flags;
pub mod start_sound;

use crate::decode::tags::common::string::String;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use sampling_bits::SamplingBits;
use sampling_rate::SamplingRate;
use sound_channels::SoundChannels;
use sound_format::SoundFormat;
use sound_info::SoundInfo;

#[derive(Clone, PartialEq, Debug)]
pub struct StartSound2Tag {
    pub sound_class_name: String,
    pub sound_info: SoundInfo,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SoundEnvelopePoint {
    pub pos_samples_44: u32,
    pub left_level: u16,
    pub right_level: u16,
}

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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum SoundStreamCompression {
    Adpcm = 1,
    Mp3 = 2,
}

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
