use super::common::{Matrix, Rectangle, Rgba};
use super::shapes::Shape;
use super::styles::{CapStyle, FillStyle, JoinStyle};

pub struct DefineMorphShapeTag {
    pub character_id: u16,
    pub start_bounds: Rectangle,
    pub end_bounds: Rectangle,
    pub morph_fill_styles: Vec<MorphFillStyle>,
    pub morph_line_styles: Vec<MorphLineStyle>,
    pub start_edges: Shape<(), ()>,
    pub end_edges: Shape<(), ()>,
}

pub struct DefineMorphShape2Tag {
    pub character_id: u16,
    pub start_bounds: Rectangle,
    pub end_bounds: Rectangle,
    pub start_edge_bounds: Rectangle,
    pub end_edge_bounds: Rectangle,
    pub uses_non_scaling_strokes: bool,
    pub uses_scaling_strokes: bool,
    pub morph_fill_styles: Vec<MorphFillStyle>,
    pub morph_line_styles: Vec<MorphLineStyle2>,
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
    pub fill_style: FillStyle<Rgba>,
}
