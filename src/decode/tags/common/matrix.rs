use crate::decode::tags::common::fixed_16::Fixed16;

/// A 2×3 matrix, used for 2D affine transformations.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Matrix {
    pub scale_x: Fixed16,
    pub scale_y: Fixed16,
    pub rotate_skew_0: Fixed16,
    pub rotate_skew_1: Fixed16,
    pub translate_x: i32,
    pub translate_y: i32,
}
