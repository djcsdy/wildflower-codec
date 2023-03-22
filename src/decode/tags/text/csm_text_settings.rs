use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::text::grid_fit::GridFit;
use crate::decode::tags::text::text_renderer::TextRenderer;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct CsmTextSettingsTag {
    pub text_id: u16,
    pub text_renderer: TextRenderer,
    pub grid_fit: GridFit,
    pub thickness: f32,
    pub sharpness: f32,
}

impl CsmTextSettingsTag {
    pub fn read(reader: &mut SwfSliceReader) -> Result<Self> {
        let text_id = reader.read_u16()?;
        let text_renderer = TextRenderer::read(reader)?;
        let grid_fit = GridFit::read(reader)?;
        reader.read_ub8(3)?;
        let thickness = reader.read_f32()?;
        let sharpness = reader.read_f32()?;
        reader.read_u8()?;
        Ok(Self {
            text_id,
            text_renderer,
            grid_fit,
            thickness,
            sharpness,
        })
    }
}
