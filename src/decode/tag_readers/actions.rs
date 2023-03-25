use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::actions::constant_pool::ConstantPool;
use crate::decode::tags::actions::define_function::DefineFunction;
use crate::decode::tags::actions::define_function_2::DefineFunction2;
use crate::decode::tags::actions::get_url::GetUrl;
use crate::decode::tags::actions::get_url_2::GetUrl2;
use crate::decode::tags::actions::go_to_frame::GoToFrame;
use crate::decode::tags::actions::go_to_frame_2::GoToFrame2;
use crate::decode::tags::actions::go_to_label::GoToLabel;
use crate::decode::tags::actions::jump::Jump;
use crate::decode::tags::actions::push::Push;
use crate::decode::tags::actions::push_value::PushValue;
use crate::decode::tags::actions::r#if::If;
use crate::decode::tags::actions::r#try::Try;
use crate::decode::tags::actions::register_param::RegisterParam;
use crate::decode::tags::actions::set_target::SetTarget;
use crate::decode::tags::actions::store_register::StoreRegister;
use crate::decode::tags::actions::wait_for_frame::WaitForFrame;
use crate::decode::tags::actions::wait_for_frame_2::WaitForFrame2;
use crate::decode::tags::actions::with::With;
use crate::decode::tags::actions::{ActionRecord, DoActionTag, DoInitActionTag};
use crate::decode::tags::common::string::String;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Result};

pub fn read_do_action_tag(reader: &mut SwfSliceReader) -> Result<DoActionTag> {
    let actions = read_action_records(reader)?;
    Ok(DoActionTag { actions })
}

pub fn read_action_record(reader: &mut SwfSliceReader) -> Result<ActionRecord> {
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
        0x8d => ActionRecord::WaitForFrame2(read_wait_for_frame_2(&mut body_reader)?),
        0x8e => ActionRecord::DefineFunction2(read_define_function_2(&mut body_reader)?),
        0x8f => ActionRecord::Try(read_try(&mut body_reader)?),
        0x94 => ActionRecord::With(read_with(&mut body_reader)?),
        0x96 => ActionRecord::Push(read_push(&mut body_reader)?),
        0x99 => ActionRecord::Jump(read_jump(&mut body_reader)?),
        0x9a => ActionRecord::GetUrl2(read_get_url_2(&mut body_reader)?),
        0x9b => ActionRecord::DefineFunction(DefineFunction::read(&mut body_reader)?),
        0x9d => ActionRecord::If(read_if(&mut body_reader)?),
        0x9e => ActionRecord::Call,
        0x9f => ActionRecord::GoToFrame2(read_go_to_frame_2(&mut body_reader)?),
        _ => return Err(Error::from(InvalidData)),
    };

    Ok(action_record)
}

pub fn read_action_records(reader: &mut SwfSliceReader) -> Result<Vec<ActionRecord>> {
    let mut actions = Vec::new();
    while reader.bytes_remaining() > 0 {
        actions.push(read_action_record(reader)?);
    }
    Ok(actions)
}

fn read_go_to_frame(reader: &mut SwfSliceReader) -> Result<GoToFrame> {
    let frame = reader.read_u16()?;
    Ok(GoToFrame { frame })
}

fn read_get_url(reader: &mut SwfSliceReader) -> Result<GetUrl> {
    let url = String::read(reader)?;
    let target = String::read(reader)?;
    Ok(GetUrl { url, target })
}

fn read_wait_for_frame(reader: &mut SwfSliceReader) -> Result<WaitForFrame> {
    let frame = reader.read_u16()?;
    let skip_count = reader.read_u8()?;
    Ok(WaitForFrame { frame, skip_count })
}

fn read_set_target(reader: &mut SwfSliceReader) -> Result<SetTarget> {
    let target_name = String::read(reader)?;
    Ok(SetTarget { target_name })
}

fn read_go_to_label(reader: &mut SwfSliceReader) -> Result<GoToLabel> {
    let label = String::read(reader)?;
    Ok(GoToLabel { label })
}

fn read_push(reader: &mut SwfSliceReader) -> Result<Push> {
    let value_type = reader.read_u8()?;
    let value = match value_type {
        0 => PushValue::String(String::read(reader)?),
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

fn read_jump(reader: &mut SwfSliceReader) -> Result<Jump> {
    let offset = reader.read_i16()?;
    Ok(Jump { offset })
}

fn read_if(reader: &mut SwfSliceReader) -> Result<If> {
    let offset = reader.read_i16()?;
    Ok(If { offset })
}

fn read_get_url_2(reader: &mut SwfSliceReader) -> Result<GetUrl2> {
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

fn read_go_to_frame_2(reader: &mut SwfSliceReader) -> Result<GoToFrame2> {
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

fn read_wait_for_frame_2(reader: &mut SwfSliceReader) -> Result<WaitForFrame2> {
    let skip_count = reader.read_u8()?;
    Ok(WaitForFrame2 { skip_count })
}

fn read_constant_pool(reader: &mut SwfSliceReader) -> Result<ConstantPool> {
    let count = reader.read_u16()?;
    let mut constant_pool = Vec::with_capacity(count as usize);
    for _ in 0..count {
        constant_pool.push(String::read(reader)?);
    }
    Ok(ConstantPool { constant_pool })
}

fn read_with(reader: &mut SwfSliceReader) -> Result<With> {
    let size = reader.read_u16()?;
    let body = read_action_records(&mut reader.slice(size as usize))?;
    Ok(With { body })
}

fn read_store_register(reader: &mut SwfSliceReader) -> Result<StoreRegister> {
    let register_number = reader.read_u8()?;
    Ok(StoreRegister { register_number })
}

pub fn read_do_init_action_tag(reader: &mut SwfSliceReader) -> Result<DoInitActionTag> {
    let sprite_id = reader.read_u16()?;
    let actions = read_action_records(reader)?;
    Ok(DoInitActionTag { sprite_id, actions })
}

fn read_define_function_2(reader: &mut SwfSliceReader) -> Result<DefineFunction2> {
    let function_name = String::read(reader)?;
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

fn read_register_param(reader: &mut SwfSliceReader) -> Result<RegisterParam> {
    let register = reader.read_u8()?;
    let name = String::read(reader)?;
    Ok(if register == 0 {
        RegisterParam::Name(name)
    } else {
        RegisterParam::Register(register)
    })
}

fn read_try(reader: &mut SwfSliceReader) -> Result<Try> {
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
        RegisterParam::Name(String::read(reader)?)
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
