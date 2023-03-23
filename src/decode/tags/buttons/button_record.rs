use crate::decode::tags::common::matrix::Matrix;

#[derive(Clone, PartialEq, Debug)]
pub struct ButtonRecord {
    pub button_state_hit_test: bool,
    pub button_state_down: bool,
    pub button_state_over: bool,
    pub button_state_up: bool,
    pub character_id: u16,
    pub place_depth: u16,
    pub place_matrix: Matrix,
}
