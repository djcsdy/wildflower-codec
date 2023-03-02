use crate::ast::display_list::{
    ClipActionRecord, ClipActions, ClipEventFlags, PlaceObject2Tag, PlaceObjectTag,
};
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag::actions::read_action_records;
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::{Read, Result};

pub fn read_place_object_tag<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<PlaceObjectTag> {
    let character_id = reader.read_u16()?;
    let depth = reader.read_u16()?;
    let matrix = reader.read_matrix()?;
    let color_transform = if reader.remaining() > 0 {
        Some(reader.read_color_transform()?)
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
        Some(reader.read_matrix()?)
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
        let mut action_record_reader = reader.with_max_length(action_record_size as usize);
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
