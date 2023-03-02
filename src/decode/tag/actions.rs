use crate::ast::actions::{
    GetUrl, GoToFrame, GoToLabel, Jump, Push, PushValue, SetTarget, WaitForFrame,
};
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read, Result};
use std::ops::Not;

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

fn read_set_target<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<SetTarget> {
    let target_name = reader.read_string()?;
    Ok(SetTarget { target_name })
}

fn read_go_to_label<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<GoToLabel> {
    let label = reader.read_string()?;
    Ok(GoToLabel { label })
}

fn read_push<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<Push> {
    let value_type = reader.read_u8()?;
    let value = match value_type {
        0 => PushValue::String(reader.read_string()?),
        1 => PushValue::Float(reader.read_f32()?),
        2 => PushValue::Null,
        3 => PushValue::Undefined,
        4 => PushValue::RegisterNumber(reader.read_u8()?),
        5 => PushValue::Boolean(reader.read_u8()? != 0),
        6 => PushValue::Double(reader.read_f64()?),
        7 => PushValue::Integer(reader.read_u32()?),
        8 => PushValue::Constant(reader.read_u8()? as u16),
        9 => PushValue::Constant(reader.read_u16()?),
        _ => return Err(Error::from(InvalidData)),
    };
    Ok(Push { value })
}

fn read_jump<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<Jump> {
    let offset = reader.read_i16()?;
    Ok(Jump { offset })
}
