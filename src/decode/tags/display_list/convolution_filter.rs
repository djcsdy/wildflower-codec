use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::common::rgba::Rgba;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct ConvolutionFilter {
    pub divisor: f32,
    pub bias: f32,
    pub matrix: Vec<Vec<f32>>,
    pub default_color: Rgba,
    pub clamp: bool,
    pub preserve_alpha: bool,
}

impl ConvolutionFilter {
    pub fn read<R: BitRead>(reader: &mut R) -> Result<Self> {
        let matrix_x = reader.read_u8()?;
        let matrix_y = reader.read_u8()?;
        let divisor = reader.read_f32()?;
        let bias = reader.read_f32()?;
        let mut matrix = vec![0f32; (matrix_x as usize) * (matrix_y as usize)];
        reader.read_f32_into(&mut matrix)?;
        let default_color = Rgba::read(reader)?;
        reader.read_ub8(6)?;
        let clamp = reader.read_bit()?;
        let preserve_alpha = reader.read_bit()?;
        Ok(Self {
            divisor,
            bias,
            matrix: matrix
                .chunks(matrix_x as usize)
                .map(|chunk| chunk.into())
                .collect(),
            default_color,
            clamp,
            preserve_alpha,
        })
    }
}
