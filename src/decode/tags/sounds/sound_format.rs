use crate::decode::bit_read::BitRead;
use crate::decode::slice_reader::SwfSliceReader;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Result};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
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

impl SoundFormat {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        SoundFormat::try_from(reader.read_ub8(4)?).map_err(|_| Error::from(InvalidData))
    }
}
