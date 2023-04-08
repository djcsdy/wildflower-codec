use crate::decode::tags::shapes::curved_edge_record::CurvedEdgeRecord;
use crate::decode::tags::shapes::straight_edge_record::StraightEdgeRecord;
use crate::decode::tags::shapes::style_change_record::StyleChangeRecord;

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
