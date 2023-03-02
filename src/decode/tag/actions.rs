use crate::ast::actions::{
    ActionRecord, ConstantPool, GetUrl, GetUrl2, GoToFrame, GoToFrame2, GoToLabel, If, Jump, Push,
    PushValue, SetTarget, WaitForFrame, WaitForFrame2,
};
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read, Result};

pub fn read_action_record<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<ActionRecord> {
    let action_code = reader.read_u8()?;
    let length = if action_code >= 0x80 {
        reader.read_u16()?
    } else {
        0
    };

    let mut body_reader = reader.with_max_length(length as usize);
    let action_record = match action_code {
        0x04 => ActionRecord::NextFrame,
        0x05 => ActionRecord::PreviousFrame,
        0x06 => ActionRecord::Play,
        0x07 => ActionRecord::Stop,
        0x08 => ActionRecord::ToggleQuality,
        0x09 => ActionRecord::StopSounds,
        0x0a => ActionRecord::Add,
        0x0b => ActionRecord::Subtract,
        0x0c => ActionRecord::Multiply,
        0x0d => ActionRecord::Divide,
        0x0e => ActionRecord::Equals,
        0x0f => ActionRecord::Less,
        0x10 => ActionRecord::And,
        0x11 => ActionRecord::Or,
        0x12 => ActionRecord::Not,
        0x13 => ActionRecord::StringEquals,
        0x14 => ActionRecord::StringLength,
        0x15 => ActionRecord::StringExtract,
        0x17 => ActionRecord::Pop,
        0x18 => ActionRecord::ToInteger,
        0x1c => ActionRecord::GetVariable,
        0x1d => ActionRecord::SetVariable,
        0x20 => ActionRecord::SetTarget2,
        0x21 => ActionRecord::StringAdd,
        0x22 => ActionRecord::GetProperty,
        0x23 => ActionRecord::SetProperty,
        0x24 => ActionRecord::CloneSprite,
        0x25 => ActionRecord::RemoveSprite,
        0x26 => ActionRecord::Trace,
        0x27 => ActionRecord::StartDrag,
        0x28 => ActionRecord::EndDrag,
        0x29 => ActionRecord::StringLess,
        0x30 => ActionRecord::RandomNumber,
        0x31 => ActionRecord::MbStringLength,
        0x32 => ActionRecord::CharToAscii,
        0x33 => ActionRecord::AsciiToChar,
        0x34 => ActionRecord::GetTime,
        0x35 => ActionRecord::MbStringExtract,
        0x36 => ActionRecord::MbCharToAscii,
        0x37 => ActionRecord::MbAsciiToChar,
        0x3d => ActionRecord::CallFunction,
        0x52 => ActionRecord::CallMethod,
        0x81 => ActionRecord::GoToFrame(read_go_to_frame(&mut body_reader)?),
        0x83 => ActionRecord::GetUrl(read_get_url(&mut body_reader)?),
        0x88 => ActionRecord::ConstantPool(read_constant_pool(&mut body_reader)?),
        0x8a => ActionRecord::WaitForFrame(read_wait_for_frame(&mut body_reader)?),
        0x8b => ActionRecord::SetTarget(read_set_target(&mut body_reader)?),
        0x8c => ActionRecord::GoToLabel(read_go_to_label(&mut body_reader)?),
        0x8d => ActionRecord::WaitForFrame2(read_wait_for_frame2(&mut body_reader)?),
        0x96 => ActionRecord::Push(read_push(&mut body_reader)?),
        0x99 => ActionRecord::Jump(read_jump(&mut body_reader)?),
        0x9a => ActionRecord::GetUrl2(read_get_url2(&mut body_reader)?),
        0x9d => ActionRecord::If(read_if(&mut body_reader)?),
        0x9e => ActionRecord::Call,
        0x9f => ActionRecord::GoToFrame2(read_go_to_frame2(&mut body_reader)?),
        _ => todo!(),
    };
    body_reader.skip_to_end()?;

    Ok(action_record)
}

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

fn read_constant_pool<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<ConstantPool> {
    let count = reader.read_u16()?;
    let mut constant_pool = Vec::with_capacity(count as usize);
    for _ in 0..count {
        constant_pool.push(reader.read_string()?);
    }
    Ok(ConstantPool { constant_pool })
}
