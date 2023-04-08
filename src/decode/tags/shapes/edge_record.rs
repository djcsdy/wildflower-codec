use crate::decode::bit_read::BitRead;
use crate::decode::tags::shapes::curved_edge_record::CurvedEdgeRecord;
use crate::decode::tags::shapes::straight_edge_record::StraightEdgeRecord;
use std::io::Result;

pub(crate) enum EdgeRecord {
    StraightEdge(StraightEdgeRecord),
    CurvedEdge(CurvedEdgeRecord),
}

impl EdgeRecord {
    pub(crate) fn read<R: BitRead>(reader: &mut R) -> Result<Self> {
        let is_straight = reader.read_bit()?;
        if is_straight {
            Ok(Self::StraightEdge(StraightEdgeRecord::read(reader)?))
        } else {
            Ok(Self::CurvedEdge(CurvedEdgeRecord::read(reader)?))
        }
    }
}
