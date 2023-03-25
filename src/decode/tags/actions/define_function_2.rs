use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::actions::action_list::ActionList;
use crate::decode::tags::actions::register_param::RegisterParam;
use crate::decode::tags::common::string::String;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFunction2 {
    pub function_name: String,
    pub register_count: u8,
    pub preload_parent: bool,
    pub preload_root: bool,
    pub suppress_super: bool,
    pub preload_super: bool,
    pub suppress_arguments: bool,
    pub preload_arguments: bool,
    pub suppress_this: bool,
    pub preload_this: bool,
    pub preload_global: bool,
    pub parameters: Vec<RegisterParam>,
    pub body: ActionList<Vec<u8>>,
}

impl DefineFunction2 {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
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
            parameters.push(RegisterParam::read(reader)?);
        }
        let code_size = reader.read_u16()?;
        let body = ActionList::read(reader, code_size as usize)?;
        Ok(Self {
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
}
