use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::io::{Error, ErrorKind::InvalidData, Result};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum LanguageCode {
    Latin = 1,
    Japanese = 2,
    Korean = 3,
    SimplifiedChinese = 4,
    TraditionalChinese = 5,
}

impl LanguageCode {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        Self::read_optional(reader)?.ok_or_else(|| Error::from(InvalidData))
    }

    pub fn read_optional(reader: &mut SwfSliceReader) -> Result<Option<Self>> {
        let code = reader.read_u8()?;
        if code == 0 {
            Ok(None)
        } else {
            Self::try_from(code)
                .map(Some)
                .map_err(|_| Error::from(InvalidData))
        }
    }
}
