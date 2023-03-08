use crate::ast::shapes::{CurvedEdgeRecord, StraightEdgeRecord, StyleChangeRecord};
use crate::ast::styles::FillStyle;
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::{Read, Result};

pub struct ReadShapeRecordOptions<
    'read_shape_record,
    R: Read,
    Color,
    LineStyle,
    ReadLineStyleArray: Fn(&mut SwfTagBodyReader<R>) -> Result<Vec<LineStyle>>,
    ReadFillStyleArray: Fn(&mut SwfTagBodyReader<R>) -> Result<Vec<FillStyle<Color>>>,
> {
    pub reader: &'read_shape_record mut SwfTagBodyReader<R>,
    pub num_fill_bits: u8,
    pub num_line_bits: u8,
    pub read_line_style_array: ReadLineStyleArray,
    pub read_fill_style_array: ReadFillStyleArray,
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
    R: Read,
    Color,
    LineStyle,
    ReadLineStyleArray: Fn(&mut SwfTagBodyReader<R>) -> Result<Vec<LineStyle>>,
    ReadFillStyleArray: Fn(&mut SwfTagBodyReader<R>) -> Result<Vec<FillStyle<Color>>>,
>(
    options: ReadShapeRecordOptions<R, Color, LineStyle, ReadLineStyleArray, ReadFillStyleArray>,
) -> Result<NonEdgeRecord<Color, LineStyle>> {
    if options.num_fill_bits > 16 || options.num_line_bits > 16 {
        panic!();
    }

    let has_new_styles = options.reader.read_bit()?;
    let has_line_style = options.reader.read_bit()?;
    let has_fill_style1 = options.reader.read_bit()?;
    let has_fill_style0 = options.reader.read_bit()?;
    let has_move_to = options.reader.read_bit()?;

    if !has_new_styles && !has_line_style && !has_fill_style1 && !has_fill_style0 && !has_move_to {
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

pub fn read_straight_edge_record<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<StraightEdgeRecord> {
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
