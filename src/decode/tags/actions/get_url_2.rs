use crate::decode::bit_read::BitRead;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::actions::load_target::LoadTarget;
use crate::decode::tags::actions::send_vars_method::SendVarsMethod;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct GetUrl2 {
    pub send_vars_method: SendVarsMethod,
    pub load_target: LoadTarget,
    pub load_variables: bool,
}

impl GetUrl2 {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
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
        Ok(Self {
            send_vars_method,
            load_target,
            load_variables,
        })
    }
}
