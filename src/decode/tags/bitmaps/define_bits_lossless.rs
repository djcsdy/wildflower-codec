use crate::decode::tags::bitmaps::BitmapData;
use crate::decode::tags::common::rgb::Rgb;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineBitsLosslessTag {
    pub character_id: u16,
    pub bitmap_width: u16,
    pub bitmap_height: u16,
    pub bitmap_data: BitmapData<Rgb>,
}
