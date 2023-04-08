use crate::decode::tags::common::matrix::Matrix;
use crate::decode::tags::common::rgba::Rgba;
use crate::decode::tags::shape_morphing::morph_gradient::MorphGradient;
use crate::decode::tags::shape_morphing::MorphFocalGradient;

#[derive(Clone, PartialEq, Debug)]
pub enum MorphFillStyle {
    Solid {
        start_color: Rgba,
        end_color: Rgba,
    },
    LinearGradient {
        start_matrix: Matrix,
        end_matrix: Matrix,
        gradient: MorphGradient,
    },
    RadialGradient {
        start_matrix: Matrix,
        end_matrix: Matrix,
        gradient: MorphGradient,
    },
    FocalRadialGradient {
        start_matrix: Matrix,
        end_matrix: Matrix,
        gradient: MorphFocalGradient,
    },
    RepeatingBitmap {
        bitmap_id: u16,
        start_matrix: Matrix,
        end_matrix: Matrix,
    },
    ClippedBitmap {
        bitmap_id: u16,
        start_matrix: Matrix,
        end_matrix: Matrix,
    },
    NonSmoothedRepeatingBitmap {
        bitmap_id: u16,
        start_matrix: Matrix,
        end_matrix: Matrix,
    },
    NonSmoothedClippedBitmap {
        bitmap_id: u16,
        start_matrix: Matrix,
        end_matrix: Matrix,
    },
}
