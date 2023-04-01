#[derive(Clone, PartialEq, Debug)]
pub struct DefineBitsTag {
    pub character_id: u16,
    pub jpeg_data: Vec<u8>,
}
