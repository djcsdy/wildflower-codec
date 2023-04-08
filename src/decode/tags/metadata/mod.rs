pub mod enable_telemetry;
pub mod file_attributes;
pub mod file_attributes_flags;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineBinaryDataTag {
    pub data: Vec<u8>,
}
