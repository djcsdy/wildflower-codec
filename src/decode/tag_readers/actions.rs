use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::actions::action_list::ActionList;
use crate::decode::tags::actions::action_record::ActionRecord;
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
use crate::decode::tags::actions::r#if::If;
use crate::decode::tags::actions::r#try::Try;
use crate::decode::tags::actions::set_target::SetTarget;
use crate::decode::tags::actions::store_register::StoreRegister;
use crate::decode::tags::actions::wait_for_frame::WaitForFrame;
use crate::decode::tags::actions::wait_for_frame_2::WaitForFrame2;
use crate::decode::tags::actions::with::With;
use crate::decode::tags::actions::{DoActionTag, DoInitActionTag};
use crate::decode::tags::common::string::String;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Result};

pub fn read_do_action_tag(reader: &mut SwfSliceReader) -> Result<DoActionTag> {
    let actions = ActionList::read_to_end(reader)?;
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
        0x81 => ActionRecord::GoToFrame(GoToFrame::read(&mut body_reader)?),
        0x83 => ActionRecord::GetUrl(GetUrl::read(&mut body_reader)?),
        0x87 => ActionRecord::StoreRegister(read_store_register(&mut body_reader)?),
        0x88 => ActionRecord::ConstantPool(read_constant_pool(&mut body_reader)?),
        0x8a => ActionRecord::WaitForFrame(WaitForFrame::read(&mut body_reader)?),
        0x8b => ActionRecord::SetTarget(SetTarget::read(&mut body_reader)?),
        0x8c => ActionRecord::GoToLabel(GoToLabel::read(&mut body_reader)?),
        0x8d => ActionRecord::WaitForFrame2(read_wait_for_frame_2(&mut body_reader)?),
        0x8e => ActionRecord::DefineFunction2(DefineFunction2::read(&mut body_reader)?),
        0x8f => ActionRecord::Try(Try::read(&mut body_reader)?),
        0x94 => ActionRecord::With(With::read(&mut body_reader)?),
        0x96 => ActionRecord::Push(Push::read(&mut body_reader)?),
        0x99 => ActionRecord::Jump(Jump::read(&mut body_reader)?),
        0x9a => ActionRecord::GetUrl2(GetUrl2::read(&mut body_reader)?),
        0x9b => ActionRecord::DefineFunction(DefineFunction::read(&mut body_reader)?),
        0x9d => ActionRecord::If(If::read(&mut body_reader)?),
        0x9e => ActionRecord::Call,
        0x9f => ActionRecord::GoToFrame2(GoToFrame2::read(&mut body_reader)?),
        _ => return Err(Error::from(InvalidData)),
    };

    Ok(action_record)
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

fn read_store_register(reader: &mut SwfSliceReader) -> Result<StoreRegister> {
    let register_number = reader.read_u8()?;
    Ok(StoreRegister { register_number })
}

pub fn read_do_init_action_tag(reader: &mut SwfSliceReader) -> Result<DoInitActionTag> {
    let sprite_id = reader.read_u16()?;
    let actions = ActionList::read_to_end(reader)?;
    Ok(DoInitActionTag { sprite_id, actions })
}
