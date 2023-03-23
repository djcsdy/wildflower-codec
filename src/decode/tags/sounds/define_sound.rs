use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::sounds::sampling_bits::SamplingBits;
use crate::decode::tags::sounds::sampling_rate::SamplingRate;
use crate::decode::tags::sounds::sound_channels::SoundChannels;
use crate::decode::tags::sounds::sound_format::SoundFormat;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineSoundTag {
    pub sound_id: u16,
    pub sound_format: SoundFormat,
    pub sound_rate: SamplingRate,
    pub sound_bits: SamplingBits,
    pub sound_channels: SoundChannels,
    pub sound_data: Vec<u8>,
}

impl DefineSoundTag {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let sound_id = reader.read_u16()?;
        let sound_format = SoundFormat::read(reader)?;
        let sound_rate = SamplingRate::read(reader)?;
        let sound_bits = SamplingBits::read(reader)?;
        let sound_channels = SoundChannels::read(reader)?;
        let sound_data = reader.read_u8_to_end()?;
        Ok(Self {
            sound_id,
            sound_format,
            sound_rate,
            sound_bits,
            sound_channels,
            sound_data,
        })
    }
}
