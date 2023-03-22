use crate::decode::tags::common::{Rectangle, Rgba, String};
use crate::decode::tags::text::define_edit_text_flags::DefineEditTextFlags;
use crate::decode::tags::text::font_ref::FontRef;
use crate::decode::tags::text::Align;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineEditTextTag {
    pub character_id: u16,
    pub bounds: Rectangle,
    pub flags: DefineEditTextFlags,
    pub font: FontRef,
    pub font_class: String,
    pub text_color: Option<Rgba>,
    pub max_length: Option<u16>,
    pub align: Align,
    pub left_margin: u16,
    pub right_margin: u16,
    pub indent: u16,
    pub leading: i16,
    pub variable_name: String,
    pub initial_text: String,
}
