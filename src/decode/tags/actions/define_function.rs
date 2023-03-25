use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::actions::action_list::ActionList;
use crate::decode::tags::common::string::String;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFunction {
    pub function_name: String,
    pub params: Vec<String>,
    pub body: ActionList<Vec<u8>>,
}

impl DefineFunction {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let function_name = String::read(reader)?;
        let num_params = reader.read_u16()?;
        let mut params = Vec::with_capacity(num_params as usize);
        for _ in 0..num_params {
            params.push(String::read(reader)?);
        }
        let code_size = reader.read_u16()?;
        let body = ActionList::read(reader, code_size as usize)?;
        Ok(Self {
            function_name,
            params,
            body,
        })
    }
}
