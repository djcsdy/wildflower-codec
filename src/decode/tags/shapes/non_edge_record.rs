use crate::decode::tags::shapes::style_change_record::StyleChangeRecord;

pub(crate) enum NonEdgeRecord<Color, LineStyle> {
    EndShape,
    StyleChange {
        style_change_record: StyleChangeRecord<Color, LineStyle>,
        num_fill_bits: u8,
        num_line_bits: u8,
    },
}
