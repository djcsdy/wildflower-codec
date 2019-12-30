pub struct FileAttributesTag {
    pub as3: bool,
    pub no_cross_domain_cache: bool,
    pub use_network: bool,
}

pub struct EnableTelemetryTag {
    pub password_hash: [u8; 32],
}

pub struct DefineBinaryDataTag {
    pub data: Vec<u8>,
}
