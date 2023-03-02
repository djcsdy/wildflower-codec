use crate::ast::actions::{GetUrl, GoToFrame, WaitForFrame};
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::{Read, Result};

fn read_go_to_frame<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<GoToFrame> {
    let frame = reader.read_u16()?;
    Ok(GoToFrame { frame })
}

fn read_get_url<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<GetUrl> {
    let url = reader.read_string()?;
    let target = reader.read_string()?;
    Ok(GetUrl { url, target })
}

fn read_wait_for_frame<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<WaitForFrame> {
    let frame = reader.read_u16()?;
    let skip_count = reader.read_u8()?;
    Ok(WaitForFrame { frame, skip_count })
}
