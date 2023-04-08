use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::common::matrix::Matrix;
use crate::decode::tags::common::rgba::Rgba;
use crate::decode::tags::shape_morphing::morph_focal_gradient::MorphFocalGradient;
use crate::decode::tags::shape_morphing::morph_gradient::MorphGradient;
use crate::decode::tags::styles::fill_style_type::FillStyleType;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub enum MorphFillStyle {
    Solid {
        start_color: Rgba,
        end_color: Rgba,
    },
    LinearGradient {
        start_matrix: Matrix,
        end_matrix: Matrix,
        gradient: MorphGradient,
    },
    RadialGradient {
        start_matrix: Matrix,
        end_matrix: Matrix,
        gradient: MorphGradient,
    },
    FocalRadialGradient {
        start_matrix: Matrix,
        end_matrix: Matrix,
        gradient: MorphFocalGradient,
    },
    RepeatingBitmap {
        bitmap_id: u16,
        start_matrix: Matrix,
        end_matrix: Matrix,
    },
    ClippedBitmap {
        bitmap_id: u16,
        start_matrix: Matrix,
        end_matrix: Matrix,
    },
    NonSmoothedRepeatingBitmap {
        bitmap_id: u16,
        start_matrix: Matrix,
        end_matrix: Matrix,
    },
    NonSmoothedClippedBitmap {
        bitmap_id: u16,
        start_matrix: Matrix,
        end_matrix: Matrix,
    },
}

impl MorphFillStyle {
    pub fn read<R: BitRead>(reader: &mut R) -> Result<Self> {
        let fill_style_type = FillStyleType::read(reader)?;
        Ok(match fill_style_type {
            FillStyleType::Solid => {
                let start_color = Rgba::read(reader)?;
                let end_color = Rgba::read(reader)?;
                Self::Solid {
                    start_color,
                    end_color,
                }
            }
            FillStyleType::LinearGradient => {
                let start_matrix = Matrix::read(reader)?;
                let end_matrix = Matrix::read(reader)?;
                let gradient = MorphGradient::read(reader)?;
                Self::LinearGradient {
                    start_matrix,
                    end_matrix,
                    gradient,
                }
            }
            FillStyleType::RadialGradient => {
                let start_matrix = Matrix::read(reader)?;
                let end_matrix = Matrix::read(reader)?;
                let gradient = MorphGradient::read(reader)?;
                Self::RadialGradient {
                    start_matrix,
                    end_matrix,
                    gradient,
                }
            }
            FillStyleType::FocalRadialGradient => {
                let start_matrix = Matrix::read(reader)?;
                let end_matrix = Matrix::read(reader)?;
                let gradient = MorphFocalGradient::read(reader)?;
                Self::FocalRadialGradient {
                    start_matrix,
                    end_matrix,
                    gradient,
                }
            }
            FillStyleType::RepeatingBitmap => {
                let bitmap_id = reader.read_u16()?;
                let start_matrix = Matrix::read(reader)?;
                let end_matrix = Matrix::read(reader)?;
                Self::RepeatingBitmap {
                    bitmap_id,
                    start_matrix,
                    end_matrix,
                }
            }
            FillStyleType::ClippedBitmap => {
                let bitmap_id = reader.read_u16()?;
                let start_matrix = Matrix::read(reader)?;
                let end_matrix = Matrix::read(reader)?;
                Self::ClippedBitmap {
                    bitmap_id,
                    start_matrix,
                    end_matrix,
                }
            }
            FillStyleType::NonSmoothedRepeatingBitmap => {
                let bitmap_id = reader.read_u16()?;
                let start_matrix = Matrix::read(reader)?;
                let end_matrix = Matrix::read(reader)?;
                Self::NonSmoothedRepeatingBitmap {
                    bitmap_id,
                    start_matrix,
                    end_matrix,
                }
            }
            FillStyleType::NonSmoothedClippedBitmap => {
                let bitmap_id = reader.read_u16()?;
                let start_matrix = Matrix::read(reader)?;
                let end_matrix = Matrix::read(reader)?;
                Self::NonSmoothedClippedBitmap {
                    bitmap_id,
                    start_matrix,
                    end_matrix,
                }
            }
        })
    }
}
