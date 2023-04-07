use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag_readers::styles;
use crate::decode::tags::common::matrix::Matrix;
use crate::decode::tags::common::rgb::Rgb;
use crate::decode::tags::styles::fill_style_type;
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
        let fill_style_type = fill_style_type::read_fill_style_type(reader)?;
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

    pub fn read_extended_array<Read: BitRead, ReadColor: Fn(&mut Read) -> Result<Color>>(
        reader: &mut Read,
        read_color: &ReadColor,
    ) -> Result<Vec<Self>> {
        let mut fill_style_count = reader.read_u8()? as u16;
        if fill_style_count == 0xff {
            fill_style_count = reader.read_u16()?;
        }
        let mut fill_styles = Vec::with_capacity(fill_style_count as usize);
        for _ in 0..fill_style_count {
            fill_styles.push(Self::read(reader, &read_color)?);
        }
        Ok(fill_styles)
    }
}

impl FillStyle<Rgb> {
    pub fn read_array<R: BitRead>(reader: &mut R) -> Result<Vec<Self>> {
        let fill_style_count = reader.read_u8()?;
        let mut fill_styles = Vec::with_capacity(fill_style_count as usize);
        for _ in 0..fill_style_count {
            let read_color = &Rgb::read;
            fill_styles.push(Self::read(reader, read_color)?);
        }
        Ok(fill_styles)
    }
}
