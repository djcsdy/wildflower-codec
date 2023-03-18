use crate::ast::header::Header;
use crate::decode::decompressing_reader::DecompressingReader;
use std::io::BufRead;

pub struct ExtractTagsReader<'reader, R: BufRead> {
    reader: DecompressingReader<&'reader mut R>,
    header: Header,
}
