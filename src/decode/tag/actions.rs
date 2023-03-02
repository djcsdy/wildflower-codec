use crate::ast::actions::GoToFrame;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::{Read, Result};

fn read_go_to_frame<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<GoToFrame> {
    let frame = reader.read_u16()?;
    Ok(GoToFrame { frame })
}
