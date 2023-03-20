use crate::decode::tags::common::Rectangle;
use crate::decode::tags::fonts::Kerning;

pub struct FontLayout<CharacterCode> {
    pub ascent: u16,
    pub descent: u16,
    pub leading: i16,
    pub advance_table: Vec<i16>,
    pub bounds_table: Vec<Rectangle>,
    pub kerning_table: Vec<Kerning<CharacterCode>>,
}