/// A Tag with an unknown or unsupported tag code.
pub struct UnknownTag {
    pub tag_code: u16,
    pub data: [u8],
}

/// A tag with a supported tag code but invalid data.
///
/// The tag does not adhere to the SWF Specification, and Wildflower was unable
/// to parse it even using known workarounds.
pub struct InvalidTag {
    pub tag_code: u16,
    pub data: [u8],
}