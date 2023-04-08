#[derive(Clone, PartialEq, Debug)]
pub struct EnableTelemetryTag {
    pub password_hash: [u8; 32],
}
