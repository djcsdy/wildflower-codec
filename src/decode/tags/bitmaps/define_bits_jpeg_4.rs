use crate::decode::tags::common::fixed_8::Fixed8;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineBitsJpeg4Tag {
    pub character_id: u16,
    pub deblock_param: Fixed8,
    pub image_data: Vec<u8>,
    pub bitmap_alpha_data: Vec<u8>,
}
