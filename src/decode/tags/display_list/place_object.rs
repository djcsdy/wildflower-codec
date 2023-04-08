use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::sized_read::SizedRead;
use crate::decode::tags::common::color_transform::ColorTransform;
use crate::decode::tags::common::matrix::Matrix;
use std::io::Result;

/// Adds a character to the display list.
#[derive(Clone, PartialEq, Debug)]
pub struct PlaceObjectTag {
    /// The ID of the character to place.
    ///
    /// The character must have previously be defined by another tag.
    pub character_id: u16,

    /// Depth of the character.
    ///
    /// Characters with higher depth values appear above characters with lower
    /// depth values.
    ///
    /// If there is already a character with this depth then the existing
    /// character is replaced.
    pub depth: u16,

    /// Affine transformation matrix applied to the character.
    pub matrix: Matrix,

    /// Color transformation applied to the character.
    pub color_transform: Option<ColorTransform>,
}

impl PlaceObjectTag {
    pub fn read<R: SizedRead + BitRead>(reader: &mut R) -> Result<Self> {
        let character_id = reader.read_u16()?;
        let depth = reader.read_u16()?;
        let matrix = Matrix::read(reader)?;
        let color_transform = if reader.remaining_bytes() > 0 {
            Some(ColorTransform::read(reader)?)
        } else {
            None
        };

        Ok(Self {
            character_id,
            depth,
            matrix,
            color_transform,
        })
    }
}
