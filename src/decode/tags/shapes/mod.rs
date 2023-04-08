pub mod define_shape;
pub mod define_shape_2;
pub mod define_shape_3;
pub mod define_shape_4;
pub mod shape;
pub mod shape_record;
pub mod shape_with_style;
pub mod straight_edge_record;
pub mod style_change_record;

#[derive(Clone, PartialEq, Debug)]
pub struct CurvedEdgeRecord {
    pub control_delta_x: i32,
    pub control_delta_y: i32,
    pub anchor_delta_x: i32,
    pub anchor_delta_y: i32,
}
