use crate::decode::tags::text::text_renderer::TextRenderer;
use crate::decode::tags::text::GridFit;

#[derive(Clone, PartialEq, Debug)]
pub struct CsmTextSettingsTag {
    pub text_id: u16,
    pub text_renderer: TextRenderer,
    pub grid_fit: GridFit,
    pub thickness: f32,
    pub sharpness: f32,
}
