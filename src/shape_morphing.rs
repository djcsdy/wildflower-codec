use super::common::{Matrix, Rectangle, Rgba};
use super::shapes::Shape;

pub struct DefineMorphShapeTag {
    pub character_id: u16,
    pub start_bounds: Rectangle,
    pub end_bounds: Rectangle,
    pub morph_fill_styles: Vec<MorphFillStyle>,
    pub morph_line_styles: Vec<MorphLineStyle>,
    pub start_edges: Shape<(), ()>,
    pub end_edges: Shape<(), ()>,
}

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
        gradient: MorphGradient,
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

pub struct MorphGradient {
    pub gradient_records: Vec<MorphGradientRecord>,
}

pub struct MorphGradientRecord {
    pub start_ratio: u8,
    pub start_color: Rgba,
    pub end_ratio: u8,
    pub end_color: Rgba,
}

pub struct MorphLineStyle {
    pub start_width: u16,
    pub end_width: u16,
    pub start_color: Rgba,
    pub end_color: Rgba,
}
