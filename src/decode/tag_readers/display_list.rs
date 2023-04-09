use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::read_options::ReadOptions;
use crate::decode::slice_read::SliceRead;
use crate::decode::tags::actions::action_list::ActionList;
use crate::decode::tags::common::color_transform_with_alpha::ColorTransformWithAlpha;
use crate::decode::tags::common::matrix::Matrix;
use crate::decode::tags::common::rgba::Rgba;
use crate::decode::tags::common::string::String;
use crate::decode::tags::display_list::blend_mode::BlendMode;
use crate::decode::tags::display_list::clip_action_record::ClipActionRecord;
use crate::decode::tags::display_list::clip_actions::ClipActions;
use crate::decode::tags::display_list::clip_event_flags::ClipEventFlags;
use crate::decode::tags::display_list::filter::Filter;
use crate::decode::tags::display_list::place_object_2::PlaceObject2Tag;
use crate::decode::tags::display_list::place_object_3::PlaceObject3Tag;
use std::io::Result;

pub fn read_place_object_2_tag<R: BitRead + SliceRead>(
    ReadOptions {
        reader,
        swf_version,
    }: ReadOptions<R>,
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
        Some(read_clip_actions(ReadOptions {
            reader,
            swf_version,
        })?)
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

fn read_clip_actions<R: SliceRead>(
    ReadOptions {
        reader,
        swf_version,
    }: ReadOptions<R>,
) -> Result<ClipActions> {
    reader.read_u16()?;
    let all_event_flags = ClipEventFlags::read(ReadOptions {
        reader,
        swf_version,
    })?;
    let mut clip_action_records = Vec::new();
    while let Some(clip_action_record) = read_clip_action_record(ReadOptions {
        reader,
        swf_version,
    })? {
        clip_action_records.push(clip_action_record);
    }
    Ok(ClipActions {
        all_event_flags,
        clip_action_records,
    })
}

fn read_clip_action_record<R: SliceRead>(
    ReadOptions {
        reader,
        swf_version,
    }: ReadOptions<R>,
) -> Result<Option<ClipActionRecord>> {
    let event_flags = ClipEventFlags::read(ReadOptions {
        reader,
        swf_version,
    })?;
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
        let actions = ActionList::read_to_end(&mut action_record_reader)?;
        Ok(Some(ClipActionRecord {
            event_flags,
            key_code,
            actions,
        }))
    }
}

pub fn read_place_object_3_tag<R: BitRead + SliceRead>(
    ReadOptions {
        reader,
        swf_version,
    }: ReadOptions<R>,
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
        Some(Filter::read_list(reader)?)
    } else {
        None
    };
    let blend_mode = if has_blend_mode {
        Some(BlendMode::read(reader)?)
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
        Some(read_clip_actions(ReadOptions {
            reader,
            swf_version,
        })?)
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
