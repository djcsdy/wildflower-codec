use crate::decode::swf_version::SwfVersion;
use std::io::Read;

pub struct ReadOptions<'reader, R: Read> {
    pub reader: &'reader mut R,
    pub swf_version: SwfVersion,
}
