use half::f16;

#[derive(Clone, PartialEq, Debug)]
pub struct ZoneData {
    pub alignment_coordinate: f16,
    pub range: f16,
}
