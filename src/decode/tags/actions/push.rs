use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::actions::push_value::PushValue;
use crate::decode::tags::common::string::String;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct Push {
    pub value: PushValue,
}

impl Push {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
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
        Ok(Self { value })
    }
}
