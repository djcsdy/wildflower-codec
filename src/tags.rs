pub struct UnknownTag {
    pub tag_code: u16,
    pub data: [u8],
}

pub struct InvalidTag {
    pub tag_code: u16,
    pub data: [u8],
}