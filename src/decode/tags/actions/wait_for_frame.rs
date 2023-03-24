#[derive(Clone, PartialEq, Debug)]
pub struct WaitForFrame {
    pub frame: u16,
    pub skip_count: u8,
}
