pub mod define_morph_shape;

use crate::decode::tags::common::fixed_8::Fixed8;
use crate::decode::tags::common::matrix::Matrix;
use crate::decode::tags::common::rectangle::Rectangle;
use crate::decode::tags::common::rgba::Rgba;
use crate::decode::tags::shapes::shape::Shape;
use crate::decode::tags::styles::cap_style::CapStyle;
use crate::decode::tags::styles::join_style::JoinStyle;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineMorphShape2Tag {
    pub character_id: u16,
    pub start_bounds: Rectangle,
    pub end_bounds: Rectangle,
    pub start_edge_bounds: Rectangle,
    pub end_edge_bounds: Rectangle,
    pub uses_non_scaling_strokes: bool,
    pub uses_scaling_strokes: bool,
    pub fill_styles: Vec<MorphFillStyle>,
    pub line_styles: Vec<MorphLineStyle2>,
    pub start_edges: Shape<(), ()>,
    pub end_edges: Shape<(), ()>,
}

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

#[derive(Clone, PartialEq, Debug)]
pub struct MorphGradient {
    pub gradient_records: Vec<MorphGradientRecord>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct MorphFocalGradient {
    pub gradient_records: Vec<MorphGradientRecord>,
    pub start_focal_point: Fixed8,
    pub end_focal_point: Fixed8,
}

#[derive(Clone, PartialEq, Debug)]
pub struct MorphGradientRecord {
    pub start_ratio: u8,
    pub start_color: Rgba,
    pub end_ratio: u8,
    pub end_color: Rgba,
}

#[derive(Clone, PartialEq, Debug)]
pub struct MorphLineStyle {
    pub start_width: u16,
    pub end_width: u16,
    pub start_color: Rgba,
    pub end_color: Rgba,
}

#[derive(Clone, PartialEq, Debug)]
pub struct MorphLineStyle2 {
    pub start_width: u16,
    pub end_width: u16,
    pub start_cap_style: CapStyle,
    pub join_style: JoinStyle,
    pub no_h_scale: bool,
    pub no_v_scale: bool,
    pub pixel_hinting: bool,
    pub no_close: bool,
    pub end_cap_style: CapStyle,
    pub fill_style: MorphFillStyle,
}
