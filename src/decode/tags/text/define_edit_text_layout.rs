use crate::decode::tags::text::align::Align;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineEditTextLayout {
    pub align: Align,
    pub left_margin: u16,
    pub right_margin: u16,
    pub indent: u16,
    pub leading: i16,
}
