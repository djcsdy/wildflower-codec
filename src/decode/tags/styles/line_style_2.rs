use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::common::fixed_8::Fixed8;
use crate::decode::tags::common::rgba::Rgba;
use crate::decode::tags::styles::cap_style::CapStyle;
use crate::decode::tags::styles::fill_style::FillStyle;
use crate::decode::tags::styles::join_style::JoinStyle;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct LineStyle2 {
    pub width: u16,
    pub start_cap_style: CapStyle,
    pub join_style: JoinStyle,
    pub no_h_scale: bool,
    pub no_v_scale: bool,
    pub pixel_hinting: bool,
    pub no_close: bool,
    pub end_cap_style: CapStyle,
    pub fill_style: FillStyle<Rgba>,
}

impl LineStyle2 {
    pub fn read<R: BitRead>(reader: &mut R) -> Result<Self> {
        let width = reader.read_u16()?;
        let start_cap_style = CapStyle::read(reader)?;
        let join_style = reader.read_ub8(2)?;
        let has_fill = reader.read_bit()?;
        let no_h_scale = reader.read_bit()?;
        let no_v_scale = reader.read_bit()?;
        let pixel_hinting = reader.read_bit()?;
        reader.read_ub8(5)?;
        let no_close = reader.read_bit()?;
        let end_cap_style = CapStyle::read(reader)?;
        let miter_limit_factor = if join_style == 2 {
            Some(Fixed8::read(reader)?)
        } else {
            None
        };
        let fill_style = if has_fill {
            FillStyle::read(reader, &Rgba::read)?
        } else {
            FillStyle::Solid(Rgba::read(reader)?)
        };
        Ok(Self {
            width,
            start_cap_style,
            join_style: match join_style {
                0 => JoinStyle::Round,
                1 => JoinStyle::Bevel,
                2 => JoinStyle::Miter {
                    miter_limit_factor: miter_limit_factor.unwrap(),
                },
                _ => return Err(Error::from(InvalidData)),
            },
            no_h_scale,
            no_v_scale,
            pixel_hinting,
            no_close,
            end_cap_style,
            fill_style,
        })
    }
}
