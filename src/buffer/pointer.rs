/// A pointer to a specific byte within a SWF file.
///
/// [ZERO][SwfPointer::ZERO] refers to the first byte of the SWF file after
/// the [Header][crate::decode::tags::header::Header].
pub struct SwfPointer(u32);

impl SwfPointer {
    /// A pointer to the first byte of a SWF file after the
    /// [Header][crate::decode::tags::header::Header].
    pub const ZERO: Self = Self(0);
}
