use crate::decode::tags::display_list::bevel_filter::BevelFilter;
use crate::decode::tags::display_list::blur_filter::BlurFilter;
use crate::decode::tags::display_list::color_matrix_filter::ColorMatrixFilter;
use crate::decode::tags::display_list::convolution_filter::ConvolutionFilter;
use crate::decode::tags::display_list::drop_shadow_filter::DropShadowFilter;
use crate::decode::tags::display_list::glow_filter::GlowFilter;
use crate::decode::tags::display_list::gradient_bevel_filter::GradientBevelFilter;
use crate::decode::tags::display_list::gradient_glow_filter::GradientGlowFilter;

#[derive(Clone, PartialEq, Debug)]
pub enum Filter {
    DropShadow(DropShadowFilter),
    Blur(BlurFilter),
    Glow(GlowFilter),
    Bevel(BevelFilter),
    GradientGlow(GradientGlowFilter),
    Convolution(ConvolutionFilter),
    ColorMatrix(ColorMatrixFilter),
    GradientBevel(GradientBevelFilter),
}
