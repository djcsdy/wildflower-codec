use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tag_readers::common::{read_rectangle, read_rgb, read_rgba};
use crate::decode::tag_readers::styles::{
    read_extended_fill_style_array, read_fill_style_array, read_line_style, read_line_style_2,
    read_line_style_array,
};
use crate::decode::tags::shapes::{
    CurvedEdgeRecord, DefineShape2Tag, DefineShape3Tag, DefineShape4Tag, DefineShapeTag, Shape,
    ShapeRecord, ShapeWithStyle, StraightEdgeRecord, StyleChangeRecord,
};
use crate::decode::tags::styles::FillStyle;
use std::io::Result;

pub fn read_shape(reader: &mut SwfSliceReader) -> Result<Shape<(), ()>> {
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
    'buffer,
    'read_line_style_array,
    'read_fill_style_array,
    Color,
    LineStyle,
    ReadLineStyleArray: Fn(&mut SwfSliceReader) -> Result<Vec<LineStyle>>,
    ReadFillStyleArray: Fn(&mut SwfSliceReader) -> Result<Vec<FillStyle<Color>>>,
> {
    pub reader: &'reader mut SwfSliceReader<'buffer>,
    pub read_line_style_array: &'read_line_style_array ReadLineStyleArray,
    pub read_fill_style_array: &'read_fill_style_array ReadFillStyleArray,
}

pub fn read_shape_with_style<
    Color,
    LineStyle,
    ReadLineStyleArray: Fn(&mut SwfSliceReader) -> Result<Vec<LineStyle>>,
    ReadFillStyleArray: Fn(&mut SwfSliceReader) -> Result<Vec<FillStyle<Color>>>,
>(
    ReadShapeWithStyleOptions {
        reader,
        read_line_style_array,
        read_fill_style_array,
    }: ReadShapeWithStyleOptions<Color, LineStyle, ReadLineStyleArray, ReadFillStyleArray>,
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
    'buffer,
    'read_line_style_array,
    'read_fill_style_array,
    Color,
    LineStyle,
    ReadLineStyleArray: Fn(&mut SwfSliceReader) -> Result<Vec<LineStyle>>,
    ReadFillStyleArray: Fn(&mut SwfSliceReader) -> Result<Vec<FillStyle<Color>>>,
> {
    pub reader: &'reader mut SwfSliceReader<'buffer>,
    pub num_fill_bits: u8,
    pub num_line_bits: u8,
    pub read_line_style_array: &'read_line_style_array ReadLineStyleArray,
    pub read_fill_style_array: &'read_fill_style_array ReadFillStyleArray,
}

fn read_shape_records<
    Color,
    LineStyle,
    ReadLineStyleArray: Fn(&mut SwfSliceReader) -> Result<Vec<LineStyle>>,
    ReadFillStyleArray: Fn(&mut SwfSliceReader) -> Result<Vec<FillStyle<Color>>>,
>(
    mut options: ReadShapeRecordOptions<Color, LineStyle, ReadLineStyleArray, ReadFillStyleArray>,
) -> Result<Vec<ShapeRecord<Color, LineStyle>>> {
    let mut shape_records = Vec::new();
    while options.reader.bytes_remaining() > 0 {
        shape_records.push(
            match read_shape_record(ReadShapeRecordOptions {
                reader: options.reader,
                num_fill_bits: options.num_fill_bits,
                num_line_bits: options.num_line_bits,
                read_line_style_array: &options.read_line_style_array,
                read_fill_style_array: &options.read_fill_style_array,
            })? {
                InternalShapeRecord::EndShape => ShapeRecord::EndShape,
                InternalShapeRecord::StyleChange {
                    style_change_record,
                    num_fill_bits,
                    num_line_bits,
                } => {
                    options.num_fill_bits = num_fill_bits;
                    options.num_line_bits = num_line_bits;
                    ShapeRecord::StyleChange(style_change_record)
                }
                InternalShapeRecord::StraightEdge(edge) => ShapeRecord::StraightEdge(edge),
                InternalShapeRecord::CurvedEdge(edge) => ShapeRecord::CurvedEdge(edge),
            },
        );
    }
    Ok(shape_records)
}

pub enum InternalShapeRecord<Color, LineStyle> {
    EndShape,
    StyleChange {
        style_change_record: StyleChangeRecord<Color, LineStyle>,
        num_fill_bits: u8,
        num_line_bits: u8,
    },
    StraightEdge(StraightEdgeRecord),
    CurvedEdge(CurvedEdgeRecord),
}

pub fn read_shape_record<
    Color,
    LineStyle,
    ReadLineStyleArray: Fn(&mut SwfSliceReader) -> Result<Vec<LineStyle>>,
    ReadFillStyleArray: Fn(&mut SwfSliceReader) -> Result<Vec<FillStyle<Color>>>,
>(
    options: ReadShapeRecordOptions<Color, LineStyle, ReadLineStyleArray, ReadFillStyleArray>,
) -> Result<InternalShapeRecord<Color, LineStyle>> {
    let is_edge = options.reader.read_bit()?;
    if is_edge {
        Ok(match read_edge_record(options.reader)? {
            EdgeRecord::StraightEdge(edge) => InternalShapeRecord::StraightEdge(edge),
            EdgeRecord::CurvedEdge(edge) => InternalShapeRecord::CurvedEdge(edge),
        })
    } else {
        Ok(match read_non_edge_record(options)? {
            NonEdgeRecord::EndShape => InternalShapeRecord::EndShape,
            NonEdgeRecord::StyleChange {
                style_change_record,
                num_fill_bits,
                num_line_bits,
            } => InternalShapeRecord::StyleChange {
                style_change_record,
                num_fill_bits,
                num_line_bits,
            },
        })
    }
}

pub enum NonEdgeRecord<Color, LineStyle> {
    EndShape,
    StyleChange {
        style_change_record: StyleChangeRecord<Color, LineStyle>,
        num_fill_bits: u8,
        num_line_bits: u8,
    },
}

pub fn read_non_edge_record<
    Color,
    LineStyle,
    ReadLineStyleArray: Fn(&mut SwfSliceReader) -> Result<Vec<LineStyle>>,
    ReadFillStyleArray: Fn(&mut SwfSliceReader) -> Result<Vec<FillStyle<Color>>>,
>(
    options: ReadShapeRecordOptions<Color, LineStyle, ReadLineStyleArray, ReadFillStyleArray>,
) -> Result<NonEdgeRecord<Color, LineStyle>> {
    if options.num_fill_bits > 16 || options.num_line_bits > 16 {
        panic!();
    }

    let has_new_styles = options.reader.read_bit()?;
    let has_line_style = options.reader.read_bit()?;
    let has_fill_style_1 = options.reader.read_bit()?;
    let has_fill_style_0 = options.reader.read_bit()?;
    let has_move_to = options.reader.read_bit()?;

    if !has_new_styles && !has_line_style && !has_fill_style_1 && !has_fill_style_0 && !has_move_to
    {
        return Ok(NonEdgeRecord::EndShape);
    }

    let move_bits = options.reader.read_ub8(5)?;
    let move_delta_x = options.reader.read_sb16(move_bits)?;
    let move_delta_y = options.reader.read_sb16(move_bits)?;
    let fill_style_0 = options.reader.read_ub16(options.num_fill_bits)?;
    let fill_style_1 = options.reader.read_ub16(options.num_fill_bits)?;
    let line_style = options.reader.read_ub16(options.num_line_bits)?;
    let fill_styles = if has_new_styles {
        Some((options.read_fill_style_array)(options.reader)?)
    } else {
        None
    };
    let line_styles = if has_new_styles {
        Some((options.read_line_style_array)(options.reader)?)
    } else {
        None
    };
    let num_fill_bits = options.reader.read_ub8(4)?;
    let num_line_bits = options.reader.read_ub8(4)?;
    Ok(NonEdgeRecord::StyleChange {
        style_change_record: StyleChangeRecord {
            move_delta: (move_delta_x, move_delta_y),
            fill_style_0,
            fill_style_1,
            line_style,
            fill_styles,
            line_styles,
        },
        num_fill_bits,
        num_line_bits,
    })
}

pub enum EdgeRecord {
    StraightEdge(StraightEdgeRecord),
    CurvedEdge(CurvedEdgeRecord),
}

pub fn read_edge_record(reader: &mut SwfSliceReader) -> Result<EdgeRecord> {
    let is_straight = reader.read_bit()?;
    if is_straight {
        Ok(EdgeRecord::StraightEdge(read_straight_edge_record(reader)?))
    } else {
        Ok(EdgeRecord::CurvedEdge(read_curved_edge_record(reader)?))
    }
}

pub fn read_straight_edge_record(reader: &mut SwfSliceReader) -> Result<StraightEdgeRecord> {
    let num_bits = reader.read_ub8(4)? + 2;
    let is_general_line = reader.read_bit()?;
    let is_vertical_line = if is_general_line {
        false
    } else {
        reader.read_bit()?
    };
    let delta_x = if is_general_line || !is_vertical_line {
        reader.read_sb(num_bits)?
    } else {
        0
    };
    let delta_y = if is_general_line || is_vertical_line {
        reader.read_sb(num_bits)?
    } else {
        0
    };
    Ok(StraightEdgeRecord { delta_x, delta_y })
}

pub fn read_curved_edge_record(reader: &mut SwfSliceReader) -> Result<CurvedEdgeRecord> {
    let num_bits = reader.read_ub8(4)? + 2;
    let control_delta_x = reader.read_sb(num_bits)?;
    let control_delta_y = reader.read_sb(num_bits)?;
    let anchor_delta_x = reader.read_sb(num_bits)?;
    let anchor_delta_y = reader.read_sb(num_bits)?;
    Ok(CurvedEdgeRecord {
        control_delta_x,
        control_delta_y,
        anchor_delta_x,
        anchor_delta_y,
    })
}

pub fn read_define_shape_tag(reader: &mut SwfSliceReader) -> Result<DefineShapeTag> {
    let shape_id = reader.read_u16()?;
    let shape_bounds = read_rectangle(reader)?;
    let shape = read_shape_with_style(ReadShapeWithStyleOptions {
        reader,
        read_line_style_array: &|reader| {
            read_line_style_array(reader, &|reader| read_line_style(reader, &read_rgb))
        },
        read_fill_style_array: &read_fill_style_array,
    })?;
    Ok(DefineShapeTag {
        shape_id,
        shape_bounds,
        shape,
    })
}

pub fn read_define_shape_2_tag(reader: &mut SwfSliceReader) -> Result<DefineShape2Tag> {
    let shape_id = reader.read_u16()?;
    let shape_bounds = read_rectangle(reader)?;
    let shape = read_shape_with_style(ReadShapeWithStyleOptions {
        reader,
        read_line_style_array: &|reader| {
            read_line_style_array(reader, &|reader| read_line_style(reader, &read_rgb))
        },
        read_fill_style_array: &|reader| read_extended_fill_style_array(reader, &read_rgb),
    })?;
    Ok(DefineShape2Tag {
        shape_id,
        shape_bounds,
        shape,
    })
}

pub fn read_define_shape_3_tag(reader: &mut SwfSliceReader) -> Result<DefineShape3Tag> {
    let shape_id = reader.read_u16()?;
    let shape_bounds = read_rectangle(reader)?;
    let shape = read_shape_with_style(ReadShapeWithStyleOptions {
        reader,
        read_line_style_array: &|reader| {
            read_line_style_array(reader, &|reader| read_line_style(reader, &read_rgba))
        },
        read_fill_style_array: &|reader| read_extended_fill_style_array(reader, &read_rgba),
    })?;
    Ok(DefineShape3Tag {
        shape_id,
        shape_bounds,
        shape,
    })
}

pub fn read_define_shape_4_tag(reader: &mut SwfSliceReader) -> Result<DefineShape4Tag> {
    let shape_id = reader.read_u16()?;
    let shape_bounds = read_rectangle(reader)?;
    let edge_bounds = read_rectangle(reader)?;
    reader.read_ub8(5)?;
    let uses_fill_winding_rule = reader.read_bit()?;
    let uses_non_scaling_strokes = reader.read_bit()?;
    let uses_scaling_strokes = reader.read_bit()?;
    let shape = read_shape_with_style(ReadShapeWithStyleOptions {
        reader,
        read_line_style_array: &|reader| read_line_style_array(reader, &read_line_style_2),
        read_fill_style_array: &|reader| read_extended_fill_style_array(reader, &read_rgba),
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
