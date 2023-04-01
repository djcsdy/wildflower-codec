#[derive(Clone, PartialEq, Debug)]
pub struct DefineBitsJpeg2Tag {
    pub character_id: u16,
    pub image_data: Vec<u8>,
}
