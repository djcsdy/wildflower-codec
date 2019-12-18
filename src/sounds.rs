pub struct DefineSoundTag {
    pub sound_id: u16,
    pub sound_format: SoundFormat,
    pub sound_rate: SamplingRate,
    pub sound_bits: SamplingBits,
    pub sound_channels: SoundChannels,
    pub sound_data: Vec<u8>,
}

pub enum SoundFormat {
    UncompressedBigEndian = 0,
    Adpcm = 1,
    Mp3 = 2,
    UncompressedLittleEndian = 3,
    Nellymoser16Khz = 4,
    Nellymoser8Khz = 5,
    Nellymoser = 6,
    Speex = 11,
}

pub enum SamplingRate {
    Khz5 = 0,
    Khz11 = 1,
    Khz22 = 2,
    Khz44 = 3,
}

pub enum SamplingBits {
    Bits8 = 0,
    Bits16 = 1,
}

pub enum SoundChannels {
    Mono = 0,
    Stereo = 1,
}
