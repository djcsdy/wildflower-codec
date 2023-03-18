/// A Tag with an unknown or unsupported tag code.
#[derive(Clone, PartialEq, Debug)]
pub struct UnknownTag {
    pub tag_code: u16,
    pub body: Vec<u8>,
}

/// A tag with a supported tag code but invalid data.
///
/// The tag does not adhere to the SWF Specification, and Wildflower was unable
/// to parse it even using known workarounds.
#[derive(Clone, PartialEq, Debug)]
pub struct InvalidTag {
    pub tag_code: u16,
    pub data: Vec<u8>,
}
