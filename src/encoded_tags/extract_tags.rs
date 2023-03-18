use crate::ast::header::Header;
use crate::decode::decompressing_reader::DecompressingReader;
use crate::decode::header::read_header;
use std::io::{BufRead, Result};

pub struct ExtractTagsReader<'reader, R: BufRead> {
    reader: DecompressingReader<&'reader mut R>,
    header: Header,
}

impl<'reader, R: BufRead> ExtractTagsReader<'reader, R> {
    pub fn extract_tags(reader: &'reader mut R) -> Result<Self> {
        read_header(reader).map(|(header, reader)| ExtractTagsReader { reader, header })
    }
}
