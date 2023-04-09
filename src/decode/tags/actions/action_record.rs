use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_read::SliceRead;
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
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Result};

#[derive(Clone, PartialEq, Debug)]
pub enum ActionRecord {
    GoToFrame(GoToFrame),
    GetUrl(GetUrl),
    NextFrame,
    PreviousFrame,
    Play,
    Stop,
    ToggleQuality,
    StopSounds,
    WaitForFrame(WaitForFrame),
    SetTarget(SetTarget),
    GoToLabel(GoToLabel),
    Push(Push),
    Pop,
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
    Less,
    And,
    Or,
    Not,
    StringEquals,
    StringLength,
    StringAdd,
    StringExtract,
    StringLess,
    MbStringLength,
    MbStringExtract,
    ToInteger,
    CharToAscii,
    AsciiToChar,
    MbCharToAscii,
    MbAsciiToChar,
    Jump(Jump),
    If(If),
    Call,
    GetVariable,
    SetVariable,
    GetUrl2(GetUrl2),
    GoToFrame2(GoToFrame2),
    SetTarget2,
    GetProperty,
    SetProperty,
    CloneSprite,
    RemoveSprite,
    StartDrag,
    EndDrag,
    WaitForFrame2(WaitForFrame2),
    Trace,
    GetTime,
    RandomNumber,
    CallFunction,
    CallMethod,
    ConstantPool(ConstantPool),
    DefineFunction(DefineFunction),
    DefineLocal,
    DefineLocal2,
    Delete,
    Delete2,
    Enumerate,
    Equals2,
    GetMember,
    InitArray,
    InitObject,
    NewMethod,
    NewObject,
    SetMember,
    TargetPath,
    With(With),
    ToNumber,
    ToString,
    TypeOf,
    Add2,
    Less2,
    Modulo,
    BitAnd,
    BitLShift,
    BitOr,
    BitRShift,
    BitURShift,
    BitXor,
    Decrement,
    Increment,
    PushDuplicate,
    Return,
    StackSwap,
    StoreRegister(StoreRegister),
    InstanceOf,
    Enumerate2,
    StrictEquals,
    Greater,
    StringGreater,
    DefineFunction2(DefineFunction2),
    Extends,
    CastOp,
    ImplementsOp,
    Try(Try),
    Throw,
}

impl ActionRecord {
    pub fn read<R: SliceRead + BitRead>(reader: &mut R) -> Result<Self> {
        let action_code = reader.read_u8()?;
        let length = if action_code >= 0x80 {
            reader.read_u16()?
        } else {
            0
        };

        let mut body_reader = reader.slice(length as usize);
        let action_record = match action_code {
            0x04 => Self::NextFrame,
            0x05 => Self::PreviousFrame,
            0x06 => Self::Play,
            0x07 => Self::Stop,
            0x08 => Self::ToggleQuality,
            0x09 => Self::StopSounds,
            0x0a => Self::Add,
            0x0b => Self::Subtract,
            0x0c => Self::Multiply,
            0x0d => Self::Divide,
            0x0e => Self::Equals,
            0x0f => Self::Less,
            0x10 => Self::And,
            0x11 => Self::Or,
            0x12 => Self::Not,
            0x13 => Self::StringEquals,
            0x14 => Self::StringLength,
            0x15 => Self::StringExtract,
            0x17 => Self::Pop,
            0x18 => Self::ToInteger,
            0x1c => Self::GetVariable,
            0x1d => Self::SetVariable,
            0x20 => Self::SetTarget2,
            0x21 => Self::StringAdd,
            0x22 => Self::GetProperty,
            0x23 => Self::SetProperty,
            0x24 => Self::CloneSprite,
            0x25 => Self::RemoveSprite,
            0x26 => Self::Trace,
            0x27 => Self::StartDrag,
            0x28 => Self::EndDrag,
            0x29 => Self::StringLess,
            0x2a => Self::Throw,
            0x2b => Self::CastOp,
            0x2c => Self::ImplementsOp,
            0x30 => Self::RandomNumber,
            0x31 => Self::MbStringLength,
            0x32 => Self::CharToAscii,
            0x33 => Self::AsciiToChar,
            0x34 => Self::GetTime,
            0x35 => Self::MbStringExtract,
            0x36 => Self::MbCharToAscii,
            0x37 => Self::MbAsciiToChar,
            0x3a => Self::Delete,
            0x3b => Self::Delete2,
            0x3c => Self::DefineLocal,
            0x3d => Self::CallFunction,
            0x3e => Self::Return,
            0x3f => Self::Modulo,
            0x40 => Self::NewObject,
            0x41 => Self::DefineLocal2,
            0x42 => Self::InitArray,
            0x43 => Self::InitObject,
            0x44 => Self::TypeOf,
            0x45 => Self::TargetPath,
            0x46 => Self::Enumerate,
            0x47 => Self::Add2,
            0x48 => Self::Less2,
            0x49 => Self::Equals2,
            0x4a => Self::ToNumber,
            0x4b => Self::ToString,
            0x4c => Self::PushDuplicate,
            0x4d => Self::StackSwap,
            0x4e => Self::GetMember,
            0x4f => Self::SetMember,
            0x50 => Self::Increment,
            0x51 => Self::Decrement,
            0x52 => Self::CallMethod,
            0x53 => Self::NewMethod,
            0x54 => Self::InstanceOf,
            0x55 => Self::Enumerate2,
            0x60 => Self::BitAnd,
            0x61 => Self::BitOr,
            0x62 => Self::BitXor,
            0x63 => Self::BitLShift,
            0x64 => Self::BitRShift,
            0x65 => Self::BitURShift,
            0x66 => Self::StrictEquals,
            0x67 => Self::Greater,
            0x68 => Self::StringGreater,
            0x69 => Self::Extends,
            0x81 => Self::GoToFrame(GoToFrame::read(&mut body_reader)?),
            0x83 => Self::GetUrl(GetUrl::read(&mut body_reader)?),
            0x87 => Self::StoreRegister(StoreRegister::read(&mut body_reader)?),
            0x88 => Self::ConstantPool(ConstantPool::read(&mut body_reader)?),
            0x8a => Self::WaitForFrame(WaitForFrame::read(&mut body_reader)?),
            0x8b => Self::SetTarget(SetTarget::read(&mut body_reader)?),
            0x8c => Self::GoToLabel(GoToLabel::read(&mut body_reader)?),
            0x8d => Self::WaitForFrame2(WaitForFrame2::read(&mut body_reader)?),
            0x8e => Self::DefineFunction2(DefineFunction2::read(&mut body_reader)?),
            0x8f => Self::Try(Try::read(&mut body_reader)?),
            0x94 => Self::With(With::read(&mut body_reader)?),
            0x96 => Self::Push(Push::read(&mut body_reader)?),
            0x99 => Self::Jump(Jump::read(&mut body_reader)?),
            0x9a => Self::GetUrl2(GetUrl2::read(&mut body_reader)?),
            0x9b => Self::DefineFunction(DefineFunction::read(&mut body_reader)?),
            0x9d => Self::If(If::read(&mut body_reader)?),
            0x9e => Self::Call,
            0x9f => Self::GoToFrame2(GoToFrame2::read(&mut body_reader)?),
            _ => return Err(Error::from(InvalidData)),
        };

        Ok(action_record)
    }
}
