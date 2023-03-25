use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::actions::action_list::ActionList;
use crate::decode::tags::actions::register_param::RegisterParam;
use crate::decode::tags::common::string;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct Try {
    pub catch_parameter: RegisterParam,
    pub try_body: ActionList<Vec<u8>>,
    pub catch_body: Option<ActionList<Vec<u8>>>,
    pub finally_body: Option<ActionList<Vec<u8>>>,
}

impl Try {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
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
            RegisterParam::Name(string::String::read(reader)?)
        };
        let try_body = ActionList::read(reader, try_size as usize)?;
        let catch_body = if has_catch_block {
            Some(ActionList::read(reader, catch_size as usize)?)
        } else {
            None
        };
        let finally_body = if has_finally_block {
            Some(ActionList::read(reader, finally_size as usize)?)
        } else {
            None
        };
        Ok(Self {
            catch_parameter,
            try_body,
            catch_body,
            finally_body,
        })
    }
}
