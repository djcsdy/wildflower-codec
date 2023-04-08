use crate::decode::tags::bitmaps::bitmap_data::BitmapData;
use crate::decode::tags::common::rgba::Rgba;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineBitsLossless2Tag {
    pub character_id: u16,
    pub bitmap_width: u16,
    pub bitmap_height: u16,
    pub bitmap_data: BitmapData<Rgba>,
}
