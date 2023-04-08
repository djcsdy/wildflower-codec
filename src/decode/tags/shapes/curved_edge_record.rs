#[derive(Clone, PartialEq, Debug)]
pub struct CurvedEdgeRecord {
    pub control_delta_x: i32,
    pub control_delta_y: i32,
    pub anchor_delta_x: i32,
    pub anchor_delta_y: i32,
}
