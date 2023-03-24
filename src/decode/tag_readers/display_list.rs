use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tag_readers::actions::read_action_records;
use crate::decode::tags::common::color_transform::ColorTransform;
use crate::decode::tags::common::color_transform_with_alpha::ColorTransformWithAlpha;
use crate::decode::tags::common::fixed_16::Fixed16;
use crate::decode::tags::common::fixed_8::Fixed8;
use crate::decode::tags::common::matrix::Matrix;
use crate::decode::tags::common::rgba::Rgba;
use crate::decode::tags::common::string::String;
use crate::decode::tags::display_list::bevel_filter::BevelFilter;
use crate::decode::tags::display_list::blur_filter::BlurFilter;
use crate::decode::tags::display_list::color_matrix_filter::ColorMatrixFilter;
use crate::decode::tags::display_list::convolution_filter::ConvolutionFilter;
use crate::decode::tags::display_list::drop_shadow_filter::DropShadowFilter;
use crate::decode::tags::display_list::filter::Filter;
use crate::decode::tags::display_list::glow_filter::GlowFilter;
use crate::decode::tags::display_list::gradient_bevel_filter::GradientBevelFilter;
use crate::decode::tags::display_list::gradient_glow_filter::GradientGlowFilter;
use crate::decode::tags::display_list::{
    ClipActionRecord, ClipActions, ClipEventFlags, PlaceObject2Tag, PlaceObject3Tag,
    PlaceObjectTag, RemoveObject2Tag, RemoveObjectTag,
};
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Result};

pub fn read_place_object_tag(reader: &mut SwfSliceReader) -> Result<PlaceObjectTag> {
    let character_id = reader.read_u16()?;
    let depth = reader.read_u16()?;
    let matrix = Matrix::read(reader)?;
    let color_transform = if reader.bytes_remaining() > 0 {
        Some(ColorTransform::read(reader)?)
    } else {
        None
    };

    Ok(PlaceObjectTag {
        character_id,
        depth,
        matrix,
        color_transform,
    })
}

pub fn read_place_object_2_tag(reader: &mut SwfSliceReader) -> Result<PlaceObject2Tag> {
    let has_clip_actions = reader.read_bit()?;
    let has_clip_depth = reader.read_bit()?;
    let has_name = reader.read_bit()?;
    let has_ratio = reader.read_bit()?;
    let has_color_transform = reader.read_bit()?;
    let has_matrix = reader.read_bit()?;
    let has_character = reader.read_bit()?;
    let modify = reader.read_bit()?;
    let depth = reader.read_u16()?;
    let character_id = if has_character {
        Some(reader.read_u16()?)
    } else {
        None
    };
    let matrix = if has_matrix {
        Some(Matrix::read(reader)?)
    } else {
        None
    };
    let color_transform = if has_color_transform {
        Some(ColorTransformWithAlpha::read(reader)?)
    } else {
        None
    };
    let ratio = if has_ratio {
        Some(reader.read_u16()?)
    } else {
        None
    };
    let name = if has_name {
        Some(String::read(reader)?)
    } else {
        None
    };
    let clip_depth = if has_clip_depth {
        Some(reader.read_u16()?)
    } else {
        None
    };
    let clip_actions = if has_clip_actions {
        Some(read_clip_actions(reader)?)
    } else {
        None
    };
    Ok(PlaceObject2Tag {
        modify,
        depth,
        character_id,
        matrix,
        color_transform,
        ratio,
        name,
        clip_depth,
        clip_actions,
    })
}

fn read_clip_actions(reader: &mut SwfSliceReader) -> Result<ClipActions> {
    reader.read_u16()?;
    let all_event_flags = read_clip_event_flags(reader)?;
    let mut clip_action_records = Vec::new();
    while let Some(clip_action_record) = read_clip_action_record(reader)? {
        clip_action_records.push(clip_action_record);
    }
    Ok(ClipActions {
        all_event_flags,
        clip_action_records,
    })
}

fn read_clip_event_flags(reader: &mut SwfSliceReader) -> Result<ClipEventFlags> {
    Ok(ClipEventFlags::from_bits_truncate(
        if reader.swf_version() >= 6 {
            reader.read_u32()?
        } else {
            reader.read_u16()? as u32
        },
    ))
}

fn read_clip_action_record(reader: &mut SwfSliceReader) -> Result<Option<ClipActionRecord>> {
    let event_flags = read_clip_event_flags(reader)?;
    if event_flags.is_empty() {
        Ok(None)
    } else {
        let action_record_size = reader.read_u32()?;
        let mut action_record_reader = reader.slice(action_record_size as usize);
        let key_code = if event_flags.contains(ClipEventFlags::KEY_PRESS) {
            Some(action_record_reader.read_u8()?)
        } else {
            None
        };
        let actions = read_action_records(&mut action_record_reader)?;
        Ok(Some(ClipActionRecord {
            event_flags,
            key_code,
            actions,
        }))
    }
}

pub fn read_place_object_3_tag(reader: &mut SwfSliceReader) -> Result<PlaceObject3Tag> {
    let has_clip_actions = reader.read_bit()?;
    let has_clip_depth = reader.read_bit()?;
    let has_name = reader.read_bit()?;
    let has_ratio = reader.read_bit()?;
    let has_color_transform = reader.read_bit()?;
    let has_matrix = reader.read_bit()?;
    let has_character = reader.read_bit()?;
    let modify = reader.read_bit()?;
    let opaque_background = reader.read_bit()?;
    let has_visible = reader.read_bit()?;
    let has_image = reader.read_bit()?;
    let has_class_name = reader.read_bit()?;
    let has_cache_as_bitmap = reader.read_bit()?;
    let has_blend_mode = reader.read_bit()?;
    let has_filter_list = reader.read_bit()?;
    let depth = reader.read_u16()?;
    let class_name = if has_class_name || (has_image && has_character) {
        Some(String::read(reader)?)
    } else {
        None
    };
    let character_id = if has_character {
        Some(reader.read_u16()?)
    } else {
        None
    };
    let matrix = if has_matrix {
        Some(Matrix::read(reader)?)
    } else {
        None
    };
    let color_transform = if has_color_transform {
        Some(ColorTransformWithAlpha::read(reader)?)
    } else {
        None
    };
    let ratio = if has_ratio {
        Some(reader.read_u16()?)
    } else {
        None
    };
    let name = if has_name {
        Some(String::read(reader)?)
    } else {
        None
    };
    let clip_depth = if has_clip_depth {
        Some(reader.read_u16()?)
    } else {
        None
    };
    let surface_filter_list = if has_filter_list {
        Some(read_filter_list(reader)?)
    } else {
        None
    };
    let blend_mode = if has_blend_mode {
        Some(
            reader
                .read_u8()?
                .try_into()
                .map_err(|_| Error::from(InvalidData))?,
        )
    } else {
        None
    };
    let bitmap_cache = if has_cache_as_bitmap {
        Some(reader.read_u8()? != 0)
    } else {
        None
    };
    let visible = if has_visible {
        Some(reader.read_u8()? != 0)
    } else {
        None
    };
    let background_color = if has_visible {
        Some(Rgba::read(reader)?)
    } else {
        None
    };
    let clip_actions = if has_clip_actions {
        Some(read_clip_actions(reader)?)
    } else {
        None
    };
    Ok(PlaceObject3Tag {
        place_object_2: PlaceObject2Tag {
            modify,
            depth,
            character_id,
            matrix,
            color_transform,
            ratio,
            name,
            clip_depth,
            clip_actions,
        },
        opaque_background,
        class_name,
        surface_filter_list,
        blend_mode,
        bitmap_cache,
        visible,
        background_color,
    })
}

fn read_filter_list(reader: &mut SwfSliceReader) -> Result<Vec<Filter>> {
    let number_of_filters = reader.read_u8()?;
    let mut filters = Vec::with_capacity(number_of_filters as usize);
    for _ in 0..number_of_filters {
        filters.push(read_filter(reader)?);
    }
    Ok(filters)
}

fn read_filter(reader: &mut SwfSliceReader) -> Result<Filter> {
    let filter_id = reader.read_u8()?;
    Ok(match filter_id {
        0 => Filter::DropShadow(DropShadowFilter::read(reader)?),
        1 => Filter::Blur(BlurFilter::read(reader)?),
        2 => Filter::Glow(GlowFilter::read(reader)?),
        3 => Filter::Bevel(BevelFilter::read(reader)?),
        4 => Filter::GradientGlow(GradientGlowFilter::read(reader)?),
        5 => Filter::Convolution(ConvolutionFilter::read(reader)?),
        6 => Filter::ColorMatrix(read_color_matrix_filter(reader)?),
        7 => Filter::GradientBevel(read_gradient_bevel_filter(reader)?),
        _ => return Err(Error::from(InvalidData)),
    })
}

fn read_color_matrix_filter(reader: &mut SwfSliceReader) -> Result<ColorMatrixFilter> {
    let mut matrix = [0f32; 20];
    reader.read_f32_into(&mut matrix)?;
    Ok(ColorMatrixFilter { matrix })
}

fn read_gradient_bevel_filter(reader: &mut SwfSliceReader) -> Result<GradientBevelFilter> {
    let num_colors = reader.read_u8()?;
    let mut colors = Vec::with_capacity(num_colors as usize);
    for _ in 0..num_colors {
        colors.push(Rgba::read(reader)?);
    }
    let mut ratio = Vec::with_capacity(num_colors as usize);
    for _ in 0..num_colors {
        ratio.push(reader.read_u8()?);
    }
    let blur_x = Fixed16::read(reader)?;
    let blur_y = Fixed16::read(reader)?;
    let angle = Fixed16::read(reader)?;
    let distance = Fixed16::read(reader)?;
    let strength = Fixed8::read(reader)?;
    let inner_shadow = reader.read_bit()?;
    let knockout = reader.read_bit()?;
    let composite_source = reader.read_bit()?;
    let on_top = reader.read_bit()?;
    let passes = reader.read_ub8(4)?;
    Ok(GradientBevelFilter {
        colors,
        ratio,
        blur_x,
        blur_y,
        angle,
        distance,
        strength,
        inner_shadow,
        knockout,
        composite_source,
        on_top,
        passes,
    })
}

pub fn read_remove_object_tag(reader: &mut SwfSliceReader) -> Result<RemoveObjectTag> {
    let character_id = reader.read_u16()?;
    let depth = reader.read_u16()?;
    Ok(RemoveObjectTag {
        character_id,
        depth,
    })
}

pub fn read_remove_object_2_tag(reader: &mut SwfSliceReader) -> Result<RemoveObject2Tag> {
    let depth = reader.read_u16()?;
    Ok(RemoveObject2Tag { depth })
}
