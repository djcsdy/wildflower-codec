use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::sounds::sampling_rate::SamplingRate;
use crate::decode::tags::sounds::sound_channels::SoundChannels;
use crate::decode::tags::sounds::sound_stream_compression::SoundStreamCompression;
use std::io::Result;

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

impl SoundStreamHeadTag {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        reader.read_ub8(4)?;
        let playback_sound_rate = SamplingRate::read(reader)?;
        reader.read_ub8(1)?;
        let playback_channels = SoundChannels::read(reader)?;
        let stream_sound_compression = SoundStreamCompression::read(reader)?;
        let stream_sound_rate = SamplingRate::read(reader)?;
        reader.read_ub8(1)?;
        let stream_sound_channels = SoundChannels::read(reader)?;
        let stream_sound_sample_count = reader.read_u16()?;
        let latency_seek = if stream_sound_compression == SoundStreamCompression::Mp3 {
            reader.read_i16()?
        } else {
            0
        };
        Ok(Self {
            playback_sound_rate,
            playback_channels,
            stream_sound_compression,
            stream_sound_rate,
            stream_sound_channels,
            stream_sound_sample_count,
            latency_seek,
        })
    }
}
