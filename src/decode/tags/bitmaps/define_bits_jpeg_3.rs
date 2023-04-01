#[derive(Clone, PartialEq, Debug)]
pub struct DefineBitsJpeg3Tag {
    pub character_id: u16,
    pub image_data: Vec<u8>,
    pub bitmap_alpha_data: Vec<u8>,
}
