use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::sized_read::SizedRead;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::common::rectangle::Rectangle;
use crate::decode::tags::common::rgb::Rgb;
use crate::decode::tags::common::rgba::Rgba;
use crate::decode::tags::shapes::define_shape::DefineShapeTag;
use crate::decode::tags::shapes::define_shape_2::DefineShape2Tag;
use crate::decode::tags::shapes::define_shape_3::DefineShape3Tag;
use crate::decode::tags::shapes::define_shape_4::DefineShape4Tag;
use crate::decode::tags::shapes::internal_shape_record::{
    InternalShapeRecord, ReadInternalShapeRecordOptions,
};
use crate::decode::tags::shapes::shape::Shape;
use crate::decode::tags::shapes::shape_record::ShapeRecord;
use crate::decode::tags::shapes::shape_with_style::ShapeWithStyle;
use crate::decode::tags::styles::fill_style::FillStyle;
use crate::decode::tags::styles::line_style::LineStyle;
use crate::decode::tags::styles::line_style_2::LineStyle2;
use crate::decode::tags::styles::line_style_array::read_line_style_array;
use std::io::Result;

pub fn read_shape<R: BitRead + SizedRead>(reader: &mut R) -> Result<Shape<(), ()>> {
    let num_fill_bits = reader.read_ub8(4)?;
    let num_line_bits = reader.read_ub8(4)?;
    let shape_records = read_shape_records(ReadShapeRecordOptions {
        reader,
        num_fill_bits,
        num_line_bits,
        read_line_style_array: &|_| Ok(vec![]),
        read_fill_style_array: &|_| Ok(vec![]),
    })?;
    Ok(Shape { shape_records })
}

pub struct ReadShapeWithStyleOptions<
    'reader,
    'read_line_style_array,
    'read_fill_style_array,
    Read: BitRead + SizedRead,
    Color,
    LineStyle,
    ReadLineStyleArray: Fn(&mut Read) -> Result<Vec<LineStyle>>,
    ReadFillStyleArray: Fn(&mut Read) -> Result<Vec<FillStyle<Color>>>,
> {
    pub reader: &'reader mut Read,
    pub read_line_style_array: &'read_line_style_array ReadLineStyleArray,
    pub read_fill_style_array: &'read_fill_style_array ReadFillStyleArray,
}

pub fn read_shape_with_style<
    Read: BitRead + SizedRead,
    Color,
    LineStyle,
    ReadLineStyleArray: Fn(&mut Read) -> Result<Vec<LineStyle>>,
    ReadFillStyleArray: Fn(&mut Read) -> Result<Vec<FillStyle<Color>>>,
>(
    ReadShapeWithStyleOptions {
        reader,
        read_line_style_array,
        read_fill_style_array,
    }: ReadShapeWithStyleOptions<
        Read,
        Color,
        LineStyle,
        ReadLineStyleArray,
        ReadFillStyleArray,
    >,
) -> Result<ShapeWithStyle<Color, LineStyle>> {
    let fill_styles = (read_fill_style_array)(reader)?;
    let line_styles = (read_line_style_array)(reader)?;
    let num_fill_bits = reader.read_ub8(4)?;
    let num_line_bits = reader.read_ub8(4)?;
    let shape_records = read_shape_records(ReadShapeRecordOptions {
        reader,
        num_fill_bits,
        num_line_bits,
        read_line_style_array,
        read_fill_style_array,
    })?;
    Ok(ShapeWithStyle {
        fill_styles,
        line_styles,
        shape_records,
    })
}

pub struct ReadShapeRecordOptions<
    'reader,
    'read_line_style_array,
    'read_fill_style_array,
    Read: BitRead + SizedRead,
    Color,
    LineStyle,
    ReadLineStyleArray: Fn(&mut Read) -> Result<Vec<LineStyle>>,
    ReadFillStyleArray: Fn(&mut Read) -> Result<Vec<FillStyle<Color>>>,
> {
    pub reader: &'reader mut Read,
    pub num_fill_bits: u8,
    pub num_line_bits: u8,
    pub read_line_style_array: &'read_line_style_array ReadLineStyleArray,
    pub read_fill_style_array: &'read_fill_style_array ReadFillStyleArray,
}

fn read_shape_records<
    Read: BitRead + SizedRead,
    Color,
    LineStyle,
    ReadLineStyleArray: Fn(&mut Read) -> Result<Vec<LineStyle>>,
    ReadFillStyleArray: Fn(&mut Read) -> Result<Vec<FillStyle<Color>>>,
>(
    ReadShapeRecordOptions {
        reader,
        mut num_fill_bits,
        mut num_line_bits,
        read_line_style_array,
        read_fill_style_array,
    }: ReadShapeRecordOptions<Read, Color, LineStyle, ReadLineStyleArray, ReadFillStyleArray>,
) -> Result<Vec<ShapeRecord<Color, LineStyle>>> {
    let mut shape_records = Vec::new();
    while reader.remaining_bytes() > 0 {
        shape_records.push(
            match InternalShapeRecord::read(ReadInternalShapeRecordOptions {
                reader,
                num_fill_bits,
                num_line_bits,
                read_line_style_array,
                read_fill_style_array,
            })? {
                InternalShapeRecord::EndShape => ShapeRecord::EndShape,
                InternalShapeRecord::StyleChange {
                    style_change_record,
                    num_fill_bits: new_num_fill_bits,
                    num_line_bits: new_num_line_bits,
                } => {
                    num_fill_bits = new_num_fill_bits;
                    num_line_bits = new_num_line_bits;
                    ShapeRecord::StyleChange(style_change_record)
                }
                InternalShapeRecord::StraightEdge(edge) => ShapeRecord::StraightEdge(edge),
                InternalShapeRecord::CurvedEdge(edge) => ShapeRecord::CurvedEdge(edge),
            },
        );
    }
    Ok(shape_records)
}

pub fn read_define_shape_tag<R: BitRead + SizedRead>(reader: &mut R) -> Result<DefineShapeTag> {
    let shape_id = reader.read_u16()?;
    let shape_bounds = Rectangle::read(reader)?;
    let shape = read_shape_with_style(ReadShapeWithStyleOptions {
        reader,
        read_line_style_array: &|reader| {
            read_line_style_array(reader, &|reader| LineStyle::read(reader, &Rgb::read))
        },
        read_fill_style_array: &|reader| FillStyle::read_array(reader),
    })?;
    Ok(DefineShapeTag {
        shape_id,
        shape_bounds,
        shape,
    })
}

pub fn read_define_shape_2_tag(reader: &mut SwfSliceReader) -> Result<DefineShape2Tag> {
    let shape_id = reader.read_u16()?;
    let shape_bounds = Rectangle::read(reader)?;
    let shape = read_shape_with_style(ReadShapeWithStyleOptions {
        reader,
        read_line_style_array: &|reader| {
            read_line_style_array(reader, &|reader| LineStyle::read(reader, &Rgb::read))
        },
        read_fill_style_array: &|reader| FillStyle::read_extended_array(reader, &Rgb::read),
    })?;
    Ok(DefineShape2Tag {
        shape_id,
        shape_bounds,
        shape,
    })
}

pub fn read_define_shape_3_tag(reader: &mut SwfSliceReader) -> Result<DefineShape3Tag> {
    let shape_id = reader.read_u16()?;
    let shape_bounds = Rectangle::read(reader)?;
    let shape = read_shape_with_style(ReadShapeWithStyleOptions {
        reader,
        read_line_style_array: &|reader| {
            read_line_style_array(reader, &|reader| LineStyle::read(reader, &Rgba::read))
        },
        read_fill_style_array: &|reader| FillStyle::read_extended_array(reader, &Rgba::read),
    })?;
    Ok(DefineShape3Tag {
        shape_id,
        shape_bounds,
        shape,
    })
}

pub fn read_define_shape_4_tag(reader: &mut SwfSliceReader) -> Result<DefineShape4Tag> {
    let shape_id = reader.read_u16()?;
    let shape_bounds = Rectangle::read(reader)?;
    let edge_bounds = Rectangle::read(reader)?;
    reader.read_ub8(5)?;
    let uses_fill_winding_rule = reader.read_bit()?;
    let uses_non_scaling_strokes = reader.read_bit()?;
    let uses_scaling_strokes = reader.read_bit()?;
    let shape = read_shape_with_style(ReadShapeWithStyleOptions {
        reader,
        read_line_style_array: &|reader| read_line_style_array(reader, &LineStyle2::read),
        read_fill_style_array: &|reader| FillStyle::read_extended_array(reader, &Rgba::read),
    })?;
    Ok(DefineShape4Tag {
        shape_id,
        shape_bounds,
        edge_bounds,
        uses_fill_winding_rule,
        uses_non_scaling_strokes,
        uses_scaling_strokes,
        shape,
    })
}
