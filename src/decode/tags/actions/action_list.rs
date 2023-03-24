#[derive(Clone, PartialEq, Debug)]
pub struct ActionList<Buffer: AsRef<[u8]>> {
    pub buffer: Buffer,
}
