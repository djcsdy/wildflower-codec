use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::text::align::Align;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineEditTextLayout {
    pub align: Align,
    pub left_margin: u16,
    pub right_margin: u16,
    pub indent: u16,
    pub leading: i16,
}

impl DefineEditTextLayout {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let align = Align::read(reader)?;
        let left_margin = reader.read_u16()?;
        let right_margin = reader.read_u16()?;
        let indent = reader.read_u16()?;
        let leading = reader.read_i16()?;
        Ok(Self {
            align,
            left_margin,
            right_margin,
            indent,
            leading,
        })
    }
}
