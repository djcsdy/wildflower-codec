pub mod file_attributes;
pub mod file_attributes_flags;

#[derive(Clone, PartialEq, Debug)]
pub struct EnableTelemetryTag {
    pub password_hash: [u8; 32],
}

#[derive(Clone, PartialEq, Debug)]
pub struct DefineBinaryDataTag {
    pub data: Vec<u8>,
}
