/// An axis-aligned rectangle.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Rectangle {
    pub x_min: i32,
    pub x_max: i32,
    pub y_min: i32,
    pub y_max: i32,
}
