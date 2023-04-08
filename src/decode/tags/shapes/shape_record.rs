use crate::decode::tags::shapes::curved_edge_record::CurvedEdgeRecord;
use crate::decode::tags::shapes::straight_edge_record::StraightEdgeRecord;
use crate::decode::tags::shapes::style_change_record::StyleChangeRecord;

#[derive(Clone, PartialEq, Debug)]
pub enum ShapeRecord<Color, LineStyle> {
    EndShape,
    StyleChange(StyleChangeRecord<Color, LineStyle>),
    StraightEdge(StraightEdgeRecord),
    CurvedEdge(CurvedEdgeRecord),
}
