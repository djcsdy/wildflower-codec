use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag_readers::styles;
use crate::decode::tags::common::matrix::Matrix;
use crate::decode::tags::styles::fill_style_type::FillStyleType;
use crate::decode::tags::styles::focal_gradient::FocalGradient;
use crate::decode::tags::styles::gradient::Gradient;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub enum FillStyle<Color> {
    Solid(Color),
    LinearGradient {
        matrix: Matrix,
        gradient: Gradient<Color>,
    },
    RadialGradient {
        matrix: Matrix,
        gradient: Gradient<Color>,
    },
    FocalRadialGradient {
        matrix: Matrix,
        gradient: FocalGradient<Color>,
    },
    RepeatingBitmap {
        bitmap_id: u16,
        matrix: Matrix,
    },
    ClippedBitmap {
        bitmap_id: u16,
        matrix: Matrix,
    },
    NonSmoothedRepeatingBitmap {
        bitmap_id: u16,
        matrix: Matrix,
    },
    NonSmoothedClippedBitmap {
        bitmap_id: u16,
        matrix: Matrix,
    },
}

impl<Color> FillStyle<Color> {
    pub fn read<Read: BitRead, ReadColor: Fn(&mut Read) -> Result<Color>>(
        reader: &mut Read,
        read_color: &ReadColor,
    ) -> Result<Self> {
        let fill_style_type = styles::read_fill_style_type(reader)?;
        Ok(match fill_style_type {
            FillStyleType::Solid => Self::Solid(read_color(reader)?),
            FillStyleType::LinearGradient => {
                let matrix = Matrix::read(reader)?;
                let gradient = styles::read_gradient(reader, &read_color)?;
                Self::LinearGradient { matrix, gradient }
            }
            FillStyleType::RadialGradient => {
                let matrix = Matrix::read(reader)?;
                let gradient = styles::read_gradient(reader, &read_color)?;
                Self::RadialGradient { matrix, gradient }
            }
            FillStyleType::FocalRadialGradient => {
                let matrix = Matrix::read(reader)?;
                let gradient = styles::read_focal_gradient(reader, &read_color)?;
                Self::FocalRadialGradient { matrix, gradient }
            }
            FillStyleType::RepeatingBitmap => {
                let bitmap_id = reader.read_u16()?;
                let matrix = Matrix::read(reader)?;
                Self::RepeatingBitmap { bitmap_id, matrix }
            }
            FillStyleType::ClippedBitmap => {
                let bitmap_id = reader.read_u16()?;
                let matrix = Matrix::read(reader)?;
                Self::ClippedBitmap { bitmap_id, matrix }
            }
            FillStyleType::NonSmoothedRepeatingBitmap => {
                let bitmap_id = reader.read_u16()?;
                let matrix = Matrix::read(reader)?;
                Self::NonSmoothedRepeatingBitmap { bitmap_id, matrix }
            }
            FillStyleType::NonSmoothedClippedBitmap => {
                let bitmap_id = reader.read_u16()?;
                let matrix = Matrix::read(reader)?;
                Self::NonSmoothedClippedBitmap { bitmap_id, matrix }
            }
        })
    }
}
