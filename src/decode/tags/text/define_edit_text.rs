use crate::decode::tags::common::{Rectangle, Rgba, String};
use crate::decode::tags::text::define_edit_text_flags::DefineEditTextFlags;
use crate::decode::tags::text::define_edit_text_layout::DefineEditTextLayout;
use crate::decode::tags::text::font_ref::FontRef;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineEditTextTag {
    pub character_id: u16,
    pub bounds: Rectangle,
    pub flags: DefineEditTextFlags,
    pub font: FontRef,
    pub font_class: String,
    pub text_color: Option<Rgba>,
    pub max_length: Option<u16>,
    pub layout: Option<DefineEditTextLayout>,
    pub variable_name: String,
    pub initial_text: String,
}
