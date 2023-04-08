use crate::decode::tags::shapes::curved_edge_record::CurvedEdgeRecord;
use crate::decode::tags::shapes::straight_edge_record::StraightEdgeRecord;

pub(crate) enum EdgeRecord {
    StraightEdge(StraightEdgeRecord),
    CurvedEdge(CurvedEdgeRecord),
}
