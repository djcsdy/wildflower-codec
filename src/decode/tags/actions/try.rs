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

bitflags! {
    struct Flags: u8 {
        const RESERVED = 0xf8;
        const CATCH_IN_REGISTER = 0x04;
        const HAS_FINALLY_BLOCK = 0x02;
        const HAS_CATCH_BLOCK = 0x01;
    }
}

impl Flags {
    fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        Ok(Self::from_bits(reader.read_u8()?).unwrap())
    }
}

impl Try {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let flags = Flags::read(reader)?;
        let try_size = reader.read_u16()?;
        let catch_size = reader.read_u16()?;
        let finally_size = reader.read_u16()?;
        let catch_parameter = if flags.contains(Flags::CATCH_IN_REGISTER) {
            RegisterParam::Register(reader.read_u8()?)
        } else {
            RegisterParam::Name(string::String::read(reader)?)
        };
        let try_body = ActionList::read(reader, try_size as usize)?;
        let catch_body = if flags.contains(Flags::HAS_CATCH_BLOCK) {
            Some(ActionList::read(reader, catch_size as usize)?)
        } else {
            None
        };
        let finally_body = if flags.contains(Flags::HAS_FINALLY_BLOCK) {
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
