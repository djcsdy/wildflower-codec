use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::common::string::String;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct ConstantPool {
    pub constant_pool: Vec<String>,
}

impl ConstantPool {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let count = reader.read_u16()?;
        let mut constant_pool = Vec::with_capacity(count as usize);
        for _ in 0..count {
            constant_pool.push(String::read(reader)?);
        }
        Ok(Self { constant_pool })
    }
}
