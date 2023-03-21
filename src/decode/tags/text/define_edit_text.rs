use crate::decode::tags::common::{Rectangle, Rgba, String};
use crate::decode::tags::text::Align;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineEditTextTag {
    pub character_id: u16,
    pub bounds: Rectangle,
    pub word_wrap: bool,
    pub multiline: bool,
    pub password: bool,
    pub read_only: bool,
    pub auto_size: bool,
    pub no_select: bool,
    pub border: bool,
    pub was_static: bool,
    pub html: bool,
    pub use_outlines: bool,
    pub font_id: u16,
    pub font_class: String,
    pub font_height: u16,
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
