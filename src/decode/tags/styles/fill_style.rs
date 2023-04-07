use crate::decode::tags::common::matrix::Matrix;
use crate::decode::tags::styles::{FocalGradient, Gradient};

#[derive(Clone, PartialEq, Debug)]
pub enum FillStyle<Color> {
    Solid(Color),
    LinearGradient {
        matrix: Matrix,
        gradient: Gradient<Color>,
    },
    RadialGradient {
        matrix: Matrix,
        gradient: Gradient<Color>,
    },
    FocalRadialGradient {
        matrix: Matrix,
        gradient: FocalGradient<Color>,
    },
    RepeatingBitmap {
        bitmap_id: u16,
        matrix: Matrix,
    },
    ClippedBitmap {
        bitmap_id: u16,
        matrix: Matrix,
    },
    NonSmoothedRepeatingBitmap {
        bitmap_id: u16,
        matrix: Matrix,
    },
    NonSmoothedClippedBitmap {
        bitmap_id: u16,
        matrix: Matrix,
    },
}
