pub struct FileAttributesTag {
    pub use_direct_blit: bool,
    pub use_gpu: bool,
    pub has_meta_data: bool,
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
