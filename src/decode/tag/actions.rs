use crate::ast::actions::{
    ActionRecord, ConstantPool, DefineFunction, DefineFunction2, DoAbcTag, DoActionTag,
    DoInitActionTag, GetUrl, GetUrl2, GoToFrame, GoToFrame2, GoToLabel, If, Jump, Push, PushValue,
    RegisterParam, SetTarget, StoreRegister, Try, WaitForFrame, WaitForFrame2, With,
};
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read, Result};

pub fn read_do_action_tag<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<DoActionTag> {
    let actions = read_action_records(reader)?;
    Ok(DoActionTag { actions })
}

pub fn read_action_record<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<ActionRecord> {
    let action_code = reader.read_u8()?;
    let length = if action_code >= 0x80 {
        reader.read_u16()?
    } else {
        0
    };

    let mut body_reader = reader.slice(length as usize);
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
        0x2a => ActionRecord::Throw,
        0x2b => ActionRecord::CastOp,
        0x2c => ActionRecord::ImplementsOp,
        0x30 => ActionRecord::RandomNumber,
        0x31 => ActionRecord::MbStringLength,
        0x32 => ActionRecord::CharToAscii,
        0x33 => ActionRecord::AsciiToChar,
        0x34 => ActionRecord::GetTime,
        0x35 => ActionRecord::MbStringExtract,
        0x36 => ActionRecord::MbCharToAscii,
        0x37 => ActionRecord::MbAsciiToChar,
        0x3a => ActionRecord::Delete,
        0x3b => ActionRecord::Delete2,
        0x3c => ActionRecord::DefineLocal,
        0x3d => ActionRecord::CallFunction,
        0x3e => ActionRecord::Return,
        0x3f => ActionRecord::Modulo,
        0x40 => ActionRecord::NewObject,
        0x41 => ActionRecord::DefineLocal2,
        0x42 => ActionRecord::InitArray,
        0x43 => ActionRecord::InitObject,
        0x44 => ActionRecord::TypeOf,
        0x45 => ActionRecord::TargetPath,
        0x46 => ActionRecord::Enumerate,
        0x47 => ActionRecord::Add2,
        0x48 => ActionRecord::Less2,
        0x49 => ActionRecord::Equals2,
        0x4a => ActionRecord::ToNumber,
        0x4b => ActionRecord::ToString,
        0x4c => ActionRecord::PushDuplicate,
        0x4d => ActionRecord::StackSwap,
        0x4e => ActionRecord::GetMember,
        0x4f => ActionRecord::SetMember,
        0x50 => ActionRecord::Increment,
        0x51 => ActionRecord::Decrement,
        0x52 => ActionRecord::CallMethod,
        0x53 => ActionRecord::NewMethod,
        0x54 => ActionRecord::InstanceOf,
        0x55 => ActionRecord::Enumerate2,
        0x60 => ActionRecord::BitAnd,
        0x61 => ActionRecord::BitOr,
        0x62 => ActionRecord::BitXor,
        0x63 => ActionRecord::BitLShift,
        0x64 => ActionRecord::BitRShift,
        0x65 => ActionRecord::BitURShift,
        0x66 => ActionRecord::StrictEquals,
        0x67 => ActionRecord::Greater,
        0x68 => ActionRecord::StringGreater,
        0x69 => ActionRecord::Extends,
        0x81 => ActionRecord::GoToFrame(read_go_to_frame(&mut body_reader)?),
        0x83 => ActionRecord::GetUrl(read_get_url(&mut body_reader)?),
        0x87 => ActionRecord::StoreRegister(read_store_register(&mut body_reader)?),
        0x88 => ActionRecord::ConstantPool(read_constant_pool(&mut body_reader)?),
        0x8a => ActionRecord::WaitForFrame(read_wait_for_frame(&mut body_reader)?),
        0x8b => ActionRecord::SetTarget(read_set_target(&mut body_reader)?),
        0x8c => ActionRecord::GoToLabel(read_go_to_label(&mut body_reader)?),
        0x8d => ActionRecord::WaitForFrame2(read_wait_for_frame2(&mut body_reader)?),
        0x8e => ActionRecord::DefineFunction2(read_define_function2(&mut body_reader)?),
        0x8f => ActionRecord::Try(read_try(&mut body_reader)?),
        0x94 => ActionRecord::With(read_with(&mut body_reader)?),
        0x96 => ActionRecord::Push(read_push(&mut body_reader)?),
        0x99 => ActionRecord::Jump(read_jump(&mut body_reader)?),
        0x9a => ActionRecord::GetUrl2(read_get_url2(&mut body_reader)?),
        0x9b => ActionRecord::DefineFunction(read_define_function(&mut body_reader)?),
        0x9d => ActionRecord::If(read_if(&mut body_reader)?),
        0x9e => ActionRecord::Call,
        0x9f => ActionRecord::GoToFrame2(read_go_to_frame2(&mut body_reader)?),
        _ => return Err(Error::from(InvalidData)),
    };
    body_reader.skip_to_end()?;

    Ok(action_record)
}

pub fn read_action_records<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<Vec<ActionRecord>> {
    let mut actions = Vec::new();
    while reader.remaining() > 0 {
        actions.push(read_action_record(reader)?);
    }
    Ok(actions)
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

fn read_define_function<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<DefineFunction> {
    let function_name = reader.read_string()?;
    let num_params = reader.read_u16()?;
    let mut params = Vec::with_capacity(num_params as usize);
    for _ in 0..num_params {
        params.push(reader.read_string()?);
    }
    let code_size = reader.read_u16()?;
    let body = read_action_records(&mut reader.slice(code_size as usize))?;
    Ok(DefineFunction {
        function_name,
        params,
        body,
    })
}

fn read_with<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<With> {
    let size = reader.read_u16()?;
    let body = read_action_records(&mut reader.slice(size as usize))?;
    Ok(With { body })
}

fn read_store_register<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<StoreRegister> {
    let register_number = reader.read_u8()?;
    Ok(StoreRegister { register_number })
}

pub fn read_do_init_action_tag<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<DoInitActionTag> {
    let sprite_id = reader.read_u16()?;
    let actions = read_action_records(reader)?;
    Ok(DoInitActionTag { sprite_id, actions })
}

fn read_define_function2<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<DefineFunction2> {
    let function_name = reader.read_string()?;
    let num_params = reader.read_u16()?;
    let register_count = reader.read_u8()?;
    let preload_parent = reader.read_bit()?;
    let preload_root = reader.read_bit()?;
    let suppress_super = reader.read_bit()?;
    let preload_super = reader.read_bit()?;
    let suppress_arguments = reader.read_bit()?;
    let preload_arguments = reader.read_bit()?;
    let suppress_this = reader.read_bit()?;
    let preload_this = reader.read_bit()?;
    reader.read_ub8(7)?;
    let preload_global = reader.read_bit()?;
    let mut parameters = Vec::with_capacity(num_params as usize);
    for _ in 0..num_params {
        parameters.push(read_register_param(reader)?);
    }
    let code_size = reader.read_u16()?;
    let body = read_action_records(&mut reader.slice(code_size as usize))?;
    Ok(DefineFunction2 {
        function_name,
        register_count,
        preload_parent,
        preload_root,
        suppress_super,
        preload_super,
        suppress_arguments,
        preload_arguments,
        suppress_this,
        preload_this,
        preload_global,
        parameters,
        body,
    })
}

fn read_register_param<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<RegisterParam> {
    let register = reader.read_u8()?;
    let name = reader.read_string()?;
    Ok(if register == 0 {
        RegisterParam::Name(name)
    } else {
        RegisterParam::Register(register)
    })
}

fn read_try<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<Try> {
    reader.read_ub8(5)?;
    let catch_in_register = reader.read_bit()?;
    let has_finally_block = reader.read_bit()?;
    let has_catch_block = reader.read_bit()?;
    let try_size = reader.read_u16()?;
    let catch_size = reader.read_u16()?;
    let finally_size = reader.read_u16()?;
    let catch_parameter = if catch_in_register {
        RegisterParam::Register(reader.read_u8()?)
    } else {
        RegisterParam::Name(reader.read_string()?)
    };
    let try_body = read_action_records(&mut reader.slice(try_size as usize))?;
    let catch_body = if has_catch_block {
        Some(read_action_records(&mut reader.slice(catch_size as usize))?)
    } else {
        None
    };
    let finally_body = if has_finally_block {
        Some(read_action_records(
            &mut reader.slice(finally_size as usize),
        )?)
    } else {
        None
    };
    Ok(Try {
        catch_parameter,
        try_body,
        catch_body,
        finally_body,
    })
}

pub fn read_do_abc_tag<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<DoAbcTag> {
    let flags = reader.read_u32()?;
    let name = reader.read_string()?;
    let abc_data = reader.read_u8_to_end()?;
    Ok(DoAbcTag {
        flags,
        name,
        abc_data,
    })
}
