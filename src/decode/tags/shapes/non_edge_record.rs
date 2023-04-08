use crate::decode::bit_read::BitRead;
use crate::decode::tags::shapes::style_change_record::StyleChangeRecord;
use crate::decode::tags::styles::fill_style::FillStyle;
use std::io::Result;

pub(crate) enum NonEdgeRecord<Color, LineStyle> {
    EndShape,
    StyleChange {
        style_change_record: StyleChangeRecord<Color, LineStyle>,
        num_fill_bits: u8,
        num_line_bits: u8,
    },
}

impl<Color, LineStyle> NonEdgeRecord<Color, LineStyle> {
    pub(crate) fn read<
        Read: BitRead,
        ReadLineStyleArray: Fn(&mut Read) -> Result<Vec<LineStyle>>,
        ReadFillStyleArray: Fn(&mut Read) -> Result<Vec<FillStyle<Color>>>,
    >(
        ReadNonEdgeRecordOptions {
            reader,
            num_fill_bits,
            num_line_bits,
            read_line_style_array,
            read_fill_style_array,
        }: ReadNonEdgeRecordOptions<
            Read,
            Color,
            LineStyle,
            ReadLineStyleArray,
            ReadFillStyleArray,
        >,
    ) -> Result<Self> {
        if num_fill_bits > 16 || num_line_bits > 16 {
            panic!();
        }

        let has_new_styles = reader.read_bit()?;
        let has_line_style = reader.read_bit()?;
        let has_fill_style_1 = reader.read_bit()?;
        let has_fill_style_0 = reader.read_bit()?;
        let has_move_to = reader.read_bit()?;

        if !has_new_styles
            && !has_line_style
            && !has_fill_style_1
            && !has_fill_style_0
            && !has_move_to
        {
            return Ok(Self::EndShape);
        }

        let move_bits = reader.read_ub8(5)?;
        let move_delta_x = reader.read_sb16(move_bits)?;
        let move_delta_y = reader.read_sb16(move_bits)?;
        let fill_style_0 = reader.read_ub16(num_fill_bits)?;
        let fill_style_1 = reader.read_ub16(num_fill_bits)?;
        let line_style = reader.read_ub16(num_line_bits)?;
        let fill_styles = if has_new_styles {
            Some(read_fill_style_array(reader)?)
        } else {
            None
        };
        let line_styles = if has_new_styles {
            Some(read_line_style_array(reader)?)
        } else {
            None
        };
        let num_fill_bits = reader.read_ub8(4)?;
        let num_line_bits = reader.read_ub8(4)?;
        Ok(Self::StyleChange {
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
}

pub(crate) struct ReadNonEdgeRecordOptions<
    'reader,
    'read_line_style_array,
    'read_fill_style_array,
    Read: BitRead,
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
