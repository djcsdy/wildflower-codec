use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tag_readers::common::read_rectangle;
use crate::decode::tags::common::rgba::Rgba;
use crate::decode::tags::common::{Rectangle, String};
use crate::decode::tags::text::define_edit_text_flags::DefineEditTextFlags;
use crate::decode::tags::text::define_edit_text_layout::DefineEditTextLayout;
use crate::decode::tags::text::font_ref::FontRef;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineEditTextTag {
    pub character_id: u16,
    pub bounds: Rectangle,
    pub flags: DefineEditTextFlags,
    pub font: Option<FontRef>,
    pub font_class: Option<String>,
    pub text_color: Option<Rgba>,
    pub max_length: Option<u16>,
    pub layout: Option<DefineEditTextLayout>,
    pub variable_name: String,
    pub initial_text: String,
}

impl DefineEditTextTag {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let character_id = reader.read_u16()?;
        let bounds = read_rectangle(reader)?;
        let flags = DefineEditTextFlags::read(reader)?;
        let font_id = if flags.contains(DefineEditTextFlags::HAS_FONT) {
            Some(reader.read_u16()?)
        } else {
            None
        };
        let font_class = if flags.contains(DefineEditTextFlags::HAS_FONT_CLASS) {
            Some(reader.read_string()?)
        } else {
            None
        };
        let font = if let Some(font_id) = font_id {
            Some(FontRef {
                font_id,
                font_height: reader.read_u16()?,
            })
        } else {
            None
        };
        let text_color = if flags.contains(DefineEditTextFlags::HAS_TEXT_COLOR) {
            Some(Rgba::read(reader)?)
        } else {
            None
        };
        let max_length = if flags.contains(DefineEditTextFlags::HAS_MAX_LENGTH) {
            Some(reader.read_u16()?)
        } else {
            None
        };
        let layout = if flags.contains(DefineEditTextFlags::HAS_LAYOUT) {
            Some(DefineEditTextLayout::read(reader)?)
        } else {
            None
        };
        let variable_name = reader.read_string()?;
        let initial_text = if flags.contains(DefineEditTextFlags::HAS_TEXT) {
            reader.read_string()?
        } else {
            String::EMPTY
        };
        Ok(Self {
            character_id,
            bounds,
            flags,
            font,
            font_class,
            text_color,
            max_length,
            layout,
            variable_name,
            initial_text,
        })
    }
}
