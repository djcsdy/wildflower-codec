use crate::ast::actions::{
    GetUrl, GetUrl2, GoToFrame, GoToFrame2, GoToLabel, If, Jump, Push, PushValue, SetTarget,
    WaitForFrame, WaitForFrame2,
};
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read, Result};

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

fn read_if<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<If> {
    let offset = reader.read_i16()?;
    Ok(If { offset })
}

fn read_get_url2<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<GetUrl2> {
    let send_vars_method = reader
        .read_ub8(2)?
        .try_into()
        .map_err(|_| Error::from(InvalidData))?;
    reader.read_ub8(4)?;
    let load_target = reader
        .read_ub8(1)?
        .try_into()
        .map_err(|_| Error::from(InvalidData))?;
    let load_variables = reader.read_bit()?;
    Ok(GetUrl2 {
        send_vars_method,
        load_target,
        load_variables,
    })
}

fn read_go_to_frame2<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<GoToFrame2> {
    reader.read_ub8(6)?;
    let scene_bias_flag = reader.read_bit()?;
    let play = reader.read_bit()?;
    let scene_bias = if scene_bias_flag {
        reader.read_u16()?
    } else {
        0
    };
    Ok(GoToFrame2 { play, scene_bias })
}

fn read_wait_for_frame2<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<WaitForFrame2> {
    let skip_count = reader.read_u8()?;
    Ok(WaitForFrame2 { skip_count })
}
