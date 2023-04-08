use crate::decode::bit_read::BitRead;
use crate::decode::sized_read::SizedRead;
use crate::decode::tags::shapes::curved_edge_record::CurvedEdgeRecord;
use crate::decode::tags::shapes::internal_shape_record::{
    InternalShapeRecord, ReadInternalShapeRecordOptions,
};
use crate::decode::tags::shapes::straight_edge_record::StraightEdgeRecord;
use crate::decode::tags::shapes::style_change_record::StyleChangeRecord;
use crate::decode::tags::styles::fill_style::FillStyle;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub enum ShapeRecord<Color, LineStyle> {
    EndShape,
    StyleChange(StyleChangeRecord<Color, LineStyle>),
    StraightEdge(StraightEdgeRecord),
    CurvedEdge(CurvedEdgeRecord),
}

impl<Color, LineStyle> ShapeRecord<Color, LineStyle> {
    pub(crate) fn read_array<
        Read: BitRead + SizedRead,
        ReadLineStyleArray: Fn(&mut Read) -> Result<Vec<LineStyle>>,
        ReadFillStyleArray: Fn(&mut Read) -> Result<Vec<FillStyle<Color>>>,
    >(
        ReadShapeRecordArrayOptions {
            reader,
            mut num_fill_bits,
            mut num_line_bits,
            read_line_style_array,
            read_fill_style_array,
        }: ReadShapeRecordArrayOptions<
            Read,
            Color,
            LineStyle,
            ReadLineStyleArray,
            ReadFillStyleArray,
        >,
    ) -> Result<Vec<Self>> {
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
                    InternalShapeRecord::EndShape => Self::EndShape,
                    InternalShapeRecord::StyleChange {
                        style_change_record,
                        num_fill_bits: new_num_fill_bits,
                        num_line_bits: new_num_line_bits,
                    } => {
                        num_fill_bits = new_num_fill_bits;
                        num_line_bits = new_num_line_bits;
                        Self::StyleChange(style_change_record)
                    }
                    InternalShapeRecord::StraightEdge(edge) => Self::StraightEdge(edge),
                    InternalShapeRecord::CurvedEdge(edge) => Self::CurvedEdge(edge),
                },
            );
        }
        Ok(shape_records)
    }
}

pub(crate) struct ReadShapeRecordArrayOptions<
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
