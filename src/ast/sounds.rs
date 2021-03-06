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

pub struct StartSoundTag {
    pub sound_id: u16,
    pub sound_info: SoundInfo,
}

pub struct StartSound2Tag {
    pub sound_class_name: String,
    pub sound_info: SoundInfo,
}

pub struct SoundInfo {
    pub sync_stop: bool,
    pub sync_no_multiple: bool,
    pub in_point: u32,
    pub out_point: u32,
    pub loop_count: u16,
    pub envelope_points: SoundEnvelopePoint,
}

pub struct SoundEnvelopePoint {
    pub pos_samples_44: u32,
    pub left_level: u16,
    pub right_level: u16,
}

pub struct SoundStreamHeadTag {
    pub playback_sound_rate: SamplingRate,
    pub playback_channels: SoundChannels,
    pub stream_sound_compression: SoundStreamCompression,
    pub stream_sound_rate: SamplingRate,
    pub stream_sound_channels: SoundChannels,
    pub stream_sound_sample_count: u16,
    pub latency_seek: i16,
}

pub enum SoundStreamCompression {
    Adpcm = 1,
    Mp3 = 2,
}

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

pub struct SoundStreamBlockTag {
    pub stream_sound_data: Vec<u8>,
}
