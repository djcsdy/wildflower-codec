#[derive(Clone, PartialEq, Debug)]
pub struct EnableTelemetryTag {
    pub password_hash: [u8; 32],
}

#[derive(Clone, PartialEq, Debug)]
pub struct DefineBinaryDataTag {
    pub data: Vec<u8>,
}
