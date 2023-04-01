use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::control::symbol_class_record::SymbolClassRecord;
use std::io::{Read, Result};

/// Creates associations between characters in the SWF file and ActionScript 3
/// classes.
#[derive(Clone, PartialEq, Debug)]
pub struct SymbolClassTag {
    pub records: Vec<SymbolClassRecord>,
}

impl SymbolClassTag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let num_symbols = reader.read_u16()?;
        let mut records = Vec::with_capacity(num_symbols as usize);
        for _ in 0..num_symbols {
            records.push(SymbolClassRecord::read(reader)?);
        }
        Ok(Self { records })
    }
}
