use crate::decode::tags::text::grid_fit::GridFit;
use crate::decode::tags::text::text_renderer::TextRenderer;

#[derive(Clone, PartialEq, Debug)]
pub struct CsmTextSettingsTag {
    pub text_id: u16,
    pub text_renderer: TextRenderer,
    pub grid_fit: GridFit,
    pub thickness: f32,
    pub sharpness: f32,
}
