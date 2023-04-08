use crate::decode::bit_read::BitRead;
use crate::decode::tags::shapes::curved_edge_record::CurvedEdgeRecord;
use crate::decode::tags::shapes::edge_record::EdgeRecord;
use crate::decode::tags::shapes::non_edge_record::{NonEdgeRecord, ReadNonEdgeRecordOptions};
use crate::decode::tags::shapes::straight_edge_record::StraightEdgeRecord;
use crate::decode::tags::shapes::style_change_record::StyleChangeRecord;
use crate::decode::tags::styles::fill_style::FillStyle;
use std::io::Result;

pub(crate) enum InternalShapeRecord<Color, LineStyle> {
    EndShape,
    StyleChange {
        style_change_record: StyleChangeRecord<Color, LineStyle>,
        num_fill_bits: u8,
        num_line_bits: u8,
    },
    StraightEdge(StraightEdgeRecord),
    CurvedEdge(CurvedEdgeRecord),
}

impl<Color, LineStyle> InternalShapeRecord<Color, LineStyle> {
    pub(crate) fn read<
        Read: BitRead,
        ReadLineStyleArray: Fn(&mut Read) -> Result<Vec<LineStyle>>,
        ReadFillStyleArray: Fn(&mut Read) -> Result<Vec<FillStyle<Color>>>,
    >(
        ReadInternalShapeRecordOptions {
            reader,
            num_fill_bits,
            num_line_bits,
            read_line_style_array,
            read_fill_style_array,
        }: ReadInternalShapeRecordOptions<
            Read,
            Color,
            LineStyle,
            ReadLineStyleArray,
            ReadFillStyleArray,
        >,
    ) -> Result<Self> {
        let is_edge = reader.read_bit()?;
        if is_edge {
            Ok(match EdgeRecord::read(reader)? {
                EdgeRecord::StraightEdge(edge) => Self::StraightEdge(edge),
                EdgeRecord::CurvedEdge(edge) => Self::CurvedEdge(edge),
            })
        } else {
            Ok(
                match NonEdgeRecord::read(ReadNonEdgeRecordOptions {
                    reader,
                    num_fill_bits,
                    num_line_bits,
                    read_line_style_array,
                    read_fill_style_array,
                })? {
                    NonEdgeRecord::EndShape => Self::EndShape,
                    NonEdgeRecord::StyleChange {
                        style_change_record,
                        num_fill_bits,
                        num_line_bits,
                    } => Self::StyleChange {
                        style_change_record,
                        num_fill_bits,
                        num_line_bits,
                    },
                },
            )
        }
    }
}

pub(crate) struct ReadInternalShapeRecordOptions<
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
