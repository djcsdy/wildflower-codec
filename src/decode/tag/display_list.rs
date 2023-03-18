use crate::ast::display_list::{
    BevelFilter, BlurFilter, ClipActionRecord, ClipActions, ClipEventFlags, ColorMatrixFilter,
    ConvolutionFilter, DropShadowFilter, Filter, GlowFilter, GradientBevelFilter,
    GradientGlowFilter, PlaceObject2Tag, PlaceObject3Tag, PlaceObjectTag, RemoveObject2Tag,
    RemoveObjectTag,
};
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag::actions::read_action_records;
use crate::decode::tag::common::{read_color_transform, read_matrix, read_rgba};
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read, Result};

pub fn read_place_object_tag<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<PlaceObjectTag> {
    let character_id = reader.read_u16()?;
    let depth = reader.read_u16()?;
    let matrix = read_matrix(reader)?;
    let color_transform = if reader.remaining() > 0 {
        Some(read_color_transform(reader)?)
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

pub fn read_place_object2_tag<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<PlaceObject2Tag> {
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
        Some(read_matrix(reader)?)
    } else {
        None
    };
    let color_transform = if has_color_transform {
        Some(reader.read_color_transform_with_alpha()?)
    } else {
        None
    };
    let ratio = if has_ratio {
        Some(reader.read_u16()?)
    } else {
        None
    };
    let name = if has_name {
        Some(reader.read_string()?)
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

fn read_clip_actions<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<ClipActions> {
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

fn read_clip_event_flags<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<ClipEventFlags> {
    Ok(ClipEventFlags::from_bits_truncate(
        if reader.swf_version() >= 6 {
            reader.read_u32()?
        } else {
            reader.read_u16()? as u32
        },
    ))
}

fn read_clip_action_record<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<Option<ClipActionRecord>> {
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

pub fn read_place_object3_tag<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<PlaceObject3Tag> {
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
        Some(reader.read_string()?)
    } else {
        None
    };
    let character_id = if has_character {
        Some(reader.read_u16()?)
    } else {
        None
    };
    let matrix = if has_matrix {
        Some(read_matrix(reader)?)
    } else {
        None
    };
    let color_transform = if has_color_transform {
        Some(reader.read_color_transform_with_alpha()?)
    } else {
        None
    };
    let ratio = if has_ratio {
        Some(reader.read_u16()?)
    } else {
        None
    };
    let name = if has_name {
        Some(reader.read_string()?)
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
        Some(read_rgba(reader)?)
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

fn read_filter_list<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<Vec<Filter>> {
    let number_of_filters = reader.read_u8()?;
    let mut filters = Vec::with_capacity(number_of_filters as usize);
    for _ in 0..number_of_filters {
        filters.push(read_filter(reader)?);
    }
    Ok(filters)
}

fn read_filter<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<Filter> {
    let filter_id = reader.read_u8()?;
    Ok(match filter_id {
        0 => Filter::DropShadow(read_drop_shadow_filter(reader)?),
        1 => Filter::Blur(read_blur_filter(reader)?),
        2 => Filter::Glow(read_glow_filter(reader)?),
        3 => Filter::Bevel(read_bevel_filter(reader)?),
        4 => Filter::GradientGlow(read_gradient_glow_filter(reader)?),
        5 => Filter::Convolution(read_convolution_filter(reader)?),
        6 => Filter::ColorMatrix(read_color_matrix_filter(reader)?),
        7 => Filter::GradientBevel(read_gradient_bevel_filter(reader)?),
        _ => return Err(Error::from(InvalidData)),
    })
}

fn read_color_matrix_filter<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<ColorMatrixFilter> {
    let mut matrix = [0f32; 20];
    reader.read_f32_into(&mut matrix)?;
    Ok(ColorMatrixFilter { matrix })
}

fn read_convolution_filter<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<ConvolutionFilter> {
    let matrix_x = reader.read_u8()?;
    let matrix_y = reader.read_u8()?;
    let divisor = reader.read_f32()?;
    let bias = reader.read_f32()?;
    let mut matrix = vec![0f32; (matrix_x as usize) * (matrix_y as usize)];
    reader.read_f32_into(&mut matrix)?;
    let default_color = read_rgba(reader)?;
    reader.read_ub8(6)?;
    let clamp = reader.read_bit()?;
    let preserve_alpha = reader.read_bit()?;
    Ok(ConvolutionFilter {
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

fn read_blur_filter<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<BlurFilter> {
    let blur_x = reader.read_fixed16()?;
    let blur_y = reader.read_fixed16()?;
    let passes = reader.read_ub8(5)?;
    reader.read_ub8(3)?;
    Ok(BlurFilter {
        blur_x,
        blur_y,
        passes,
    })
}

fn read_drop_shadow_filter<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<DropShadowFilter> {
    let color = read_rgba(reader)?;
    let blur_x = reader.read_fixed16()?;
    let blur_y = reader.read_fixed16()?;
    let angle = reader.read_fixed16()?;
    let distance = reader.read_fixed16()?;
    let strength = reader.read_fixed8()?;
    let inner_shadow = reader.read_bit()?;
    let knockout = reader.read_bit()?;
    let composite_source = reader.read_bit()?;
    let passes = reader.read_ub8(5)?;
    Ok(DropShadowFilter {
        color,
        blur_x,
        blur_y,
        angle,
        distance,
        strength,
        inner_shadow,
        knockout,
        composite_source,
        passes,
    })
}

fn read_glow_filter<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<GlowFilter> {
    let color = read_rgba(reader)?;
    let blur_x = reader.read_fixed16()?;
    let blur_y = reader.read_fixed16()?;
    let strength = reader.read_fixed8()?;
    let inner_glow = reader.read_bit()?;
    let knockout = reader.read_bit()?;
    let composite_source = reader.read_bit()?;
    let passes = reader.read_ub8(5)?;
    Ok(GlowFilter {
        color,
        blur_x,
        blur_y,
        strength,
        inner_glow,
        knockout,
        composite_source,
        passes,
    })
}

fn read_bevel_filter<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<BevelFilter> {
    let shadow_color = read_rgba(reader)?;
    let highlight_color = read_rgba(reader)?;
    let blur_x = reader.read_fixed16()?;
    let blur_y = reader.read_fixed16()?;
    let angle = reader.read_fixed16()?;
    let distance = reader.read_fixed16()?;
    let strength = reader.read_fixed8()?;
    let inner_shadow = reader.read_bit()?;
    let knockout = reader.read_bit()?;
    let composite_source = reader.read_bit()?;
    let on_top = reader.read_bit()?;
    let passes = reader.read_ub8(4)?;
    Ok(BevelFilter {
        shadow_color,
        highlight_color,
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

fn read_gradient_glow_filter<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<GradientGlowFilter> {
    let num_colors = reader.read_u8()?;
    let mut colors = Vec::with_capacity(num_colors as usize);
    for _ in 0..num_colors {
        colors.push(read_rgba(reader)?);
    }
    let mut ratio = Vec::with_capacity(num_colors as usize);
    for _ in 0..num_colors {
        ratio.push(reader.read_u8()?);
    }
    let blur_x = reader.read_fixed16()?;
    let blur_y = reader.read_fixed16()?;
    let angle = reader.read_fixed16()?;
    let distance = reader.read_fixed16()?;
    let strength = reader.read_fixed8()?;
    let inner_shadow = reader.read_bit()?;
    let knockout = reader.read_bit()?;
    let composite_source = reader.read_bit()?;
    let on_top = reader.read_bit()?;
    let passes = reader.read_ub8(4)?;
    Ok(GradientGlowFilter {
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

fn read_gradient_bevel_filter<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<GradientBevelFilter> {
    let num_colors = reader.read_u8()?;
    let mut colors = Vec::with_capacity(num_colors as usize);
    for _ in 0..num_colors {
        colors.push(read_rgba(reader)?);
    }
    let mut ratio = Vec::with_capacity(num_colors as usize);
    for _ in 0..num_colors {
        ratio.push(reader.read_u8()?);
    }
    let blur_x = reader.read_fixed16()?;
    let blur_y = reader.read_fixed16()?;
    let angle = reader.read_fixed16()?;
    let distance = reader.read_fixed16()?;
    let strength = reader.read_fixed8()?;
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

pub fn read_remove_object_tag<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<RemoveObjectTag> {
    let character_id = reader.read_u16()?;
    let depth = reader.read_u16()?;
    Ok(RemoveObjectTag {
        character_id,
        depth,
    })
}

pub fn read_remove_object2_tag<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<RemoveObject2Tag> {
    let depth = reader.read_u16()?;
    Ok(RemoveObject2Tag { depth })
}
