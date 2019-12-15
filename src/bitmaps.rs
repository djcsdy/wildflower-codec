pub struct DefineBitsTag {
    pub character_id: u16,
    pub jpeg_data: Vec<u8>,
}

pub struct JpegTablesTag {
    pub jpeg_data: Vec<u8>,
}

pub struct DefineBitsJpeg2Tag {
    pub character_id: u16,
    pub image_data: Vec<u8>,
}

pub struct DefineBitsJpeg3Tag {
    pub character_id: u16,
    pub image_data: Vec<u8>,
    pub bitmap_alpha_data: Vec<u8>,
}
