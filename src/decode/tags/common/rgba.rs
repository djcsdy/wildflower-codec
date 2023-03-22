/// An RGB color with an alpha component.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Rgba {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
