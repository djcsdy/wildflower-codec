use crate::decode::decompressing_reader::DecompressingReader;
use crate::decode::header::Header;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::swf_version::SwfVersion;
use crate::extract::encoded_tag::EncodedTag;
use crate::extract::tag_type::TagType;
use std::io::{BufRead, Result};

pub struct ExtractTagsReader<'reader, R: BufRead> {
    reader: DecompressingReader<&'reader mut R>,
    header: Header,
}

impl<'reader, R: BufRead> ExtractTagsReader<'reader, R> {
    pub fn extract_tags(reader: &'reader mut R) -> Result<Self> {
        Header::read(reader).map(|(header, reader)| ExtractTagsReader { reader, header })
    }

    pub fn read_tag(&mut self) -> Result<EncodedTag> {
        let tag_code_and_length = self.reader.read_u16()?;
        let tag_type = TagType::from(tag_code_and_length >> 6);
        let mut length = (tag_code_and_length & 0x3f) as usize;
        if length == 0x3f {
            length = self.reader.read_u32()? as usize
        }

        let mut body = vec![0u8; length];
        self.reader.read_u8_into(&mut body)?;

        Ok(EncodedTag::new(
            SwfVersion(self.header.version),
            tag_type,
            body,
        ))
    }
}
