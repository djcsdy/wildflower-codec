use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::display_list::bevel_filter::BevelFilter;
use crate::decode::tags::display_list::blur_filter::BlurFilter;
use crate::decode::tags::display_list::color_matrix_filter::ColorMatrixFilter;
use crate::decode::tags::display_list::convolution_filter::ConvolutionFilter;
use crate::decode::tags::display_list::drop_shadow_filter::DropShadowFilter;
use crate::decode::tags::display_list::glow_filter::GlowFilter;
use crate::decode::tags::display_list::gradient_bevel_filter::GradientBevelFilter;
use crate::decode::tags::display_list::gradient_glow_filter::GradientGlowFilter;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Result};

#[derive(Clone, PartialEq, Debug)]
pub enum Filter {
    DropShadow(DropShadowFilter),
    Blur(BlurFilter),
    Glow(GlowFilter),
    Bevel(BevelFilter),
    GradientGlow(GradientGlowFilter),
    Convolution(ConvolutionFilter),
    ColorMatrix(ColorMatrixFilter),
    GradientBevel(GradientBevelFilter),
}

impl Filter {
    pub fn read<R: BitRead>(reader: &mut R) -> Result<Filter> {
        let filter_id = reader.read_u8()?;
        Ok(match filter_id {
            0 => Filter::DropShadow(DropShadowFilter::read(reader)?),
            1 => Filter::Blur(BlurFilter::read(reader)?),
            2 => Filter::Glow(GlowFilter::read(reader)?),
            3 => Filter::Bevel(BevelFilter::read(reader)?),
            4 => Filter::GradientGlow(GradientGlowFilter::read(reader)?),
            5 => Filter::Convolution(ConvolutionFilter::read(reader)?),
            6 => Filter::ColorMatrix(ColorMatrixFilter::read(reader)?),
            7 => Filter::GradientBevel(GradientBevelFilter::read(reader)?),
            _ => return Err(Error::from(InvalidData)),
        })
    }

    pub fn read_list(reader: &mut SwfSliceReader) -> Result<Vec<Filter>> {
        let number_of_filters = reader.read_u8()?;
        let mut filters = Vec::with_capacity(number_of_filters as usize);
        for _ in 0..number_of_filters {
            filters.push(Filter::read(reader)?);
        }
        Ok(filters)
    }
}
