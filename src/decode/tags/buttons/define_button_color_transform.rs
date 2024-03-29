use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::common::color_transform::ColorTransform;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineButtonColorTransformTag {
    pub button_id: u16,
    pub button_color_transform: ColorTransform,
}

impl DefineButtonColorTransformTag {
    pub fn read<R: BitRead>(reader: &mut R) -> Result<Self> {
        let button_id = reader.read_u16()?;
        let button_color_transform = ColorTransform::read(reader)?;
        Ok(Self {
            button_id,
            button_color_transform,
        })
    }
}
