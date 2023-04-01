use std::io::Read;
use std::io;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::common::string::String;
use crate::decode::tags::control::portable_character_record::PortableCharacterRecord;

/// Imports characters from another SWF file.
#[derive(Clone, PartialEq, Debug)]
pub struct ImportAssetsTag {
    /// URL of the source SWF file.
    pub url: String,

    pub imports: Vec<PortableCharacterRecord>,
}

impl ImportAssetsTag {
    pub fn read<R: Read>(reader: &mut R) -> io::Result<ImportAssetsTag> {
        let url = String::read(reader)?;
        let count = reader.read_u16()?;
        let mut imports = Vec::with_capacity(count as usize);
        for _ in 0..count {
            imports.push(PortableCharacterRecord::read(reader)?);
        }
        Ok(ImportAssetsTag { url, imports })
    }
}