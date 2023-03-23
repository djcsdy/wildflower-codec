use crate::decode::tags::buttons::button_record_flags::ButtonRecordFlags;
use crate::decode::tags::common::matrix::Matrix;

#[derive(Clone, PartialEq, Debug)]
pub struct ButtonRecord {
    pub flags: ButtonRecordFlags,
    pub character_id: u16,
    pub place_depth: u16,
    pub place_matrix: Matrix,
}
