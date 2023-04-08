use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::common::fixed_8::Fixed8;
use crate::decode::tags::common::rgba::Rgba;
use crate::decode::tags::shape_morphing::morph_fill_style::MorphFillStyle;
use crate::decode::tags::styles::cap_style::CapStyle;
use crate::decode::tags::styles::join_style::JoinStyle;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct MorphLineStyle2 {
    pub start_width: u16,
    pub end_width: u16,
    pub start_cap_style: CapStyle,
    pub join_style: JoinStyle,
    pub no_h_scale: bool,
    pub no_v_scale: bool,
    pub pixel_hinting: bool,
    pub no_close: bool,
    pub end_cap_style: CapStyle,
    pub fill_style: MorphFillStyle,
}

impl MorphLineStyle2 {
    pub fn read<R: BitRead>(reader: &mut R) -> Result<Self> {
        let start_width = reader.read_u16()?;
        let end_width = reader.read_u16()?;
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
            MorphFillStyle::read(reader)?
        } else {
            let start_color = Rgba::read(reader)?;
            let end_color = Rgba::read(reader)?;
            MorphFillStyle::Solid {
                start_color,
                end_color,
            }
        };
        Ok(Self {
            start_width,
            end_width,
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
