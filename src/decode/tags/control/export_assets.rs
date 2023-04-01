use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::control::portable_character_record::PortableCharacterRecord;
use std::io::{Read, Result};

/// Makes portions of the SWF file available for import by other SWF files
/// using `ImportAssetsTag`.
#[derive(Clone, PartialEq, Debug)]
pub struct ExportAssetsTag {
    pub exports: Vec<PortableCharacterRecord>,
}

impl ExportAssetsTag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let count = reader.read_u16()?;
        let mut exports = Vec::with_capacity(count as usize);
        for _ in 0..count {
            exports.push(PortableCharacterRecord::read(reader)?);
        }
        Ok(Self { exports })
    }
}
