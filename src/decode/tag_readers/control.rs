use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::common::rectangle::Rectangle;
use crate::decode::tags::common::string::String;
use crate::decode::tags::control::define_scaling_grid::DefineScalingGridTag;
use crate::decode::tags::control::define_scene_and_frame_label_data::DefineSceneAndFrameLabelDataTag;
use crate::decode::tags::control::frame_label::FrameLabelTag;
use crate::decode::tags::control::frame_label_record::FrameLabelRecord;
use crate::decode::tags::control::scene_record::SceneRecord;
use crate::decode::tags::metadata::{FileAttributesFlags, FileAttributesTag};
use std::io::{Read, Result};

pub fn read_frame_label_tag(reader: &mut SwfSliceReader) -> Result<FrameLabelTag> {
    let name = String::read(reader)?;
    let named_anchor = reader.bytes_remaining() > 0 && reader.read_u8()? == 1;
    Ok(FrameLabelTag { name, named_anchor })
}

pub fn read_file_attributes_tag<R: Read>(reader: &mut R) -> Result<FileAttributesTag> {
    let flags = FileAttributesFlags::from_bits_truncate(reader.read_u32()?);
    Ok(FileAttributesTag { flags })
}

pub fn read_define_scaling_grid_tag<R: BitRead>(reader: &mut R) -> Result<DefineScalingGridTag> {
    let character_id = reader.read_u16()?;
    let splitter = Rectangle::read(reader)?;
    Ok(DefineScalingGridTag {
        character_id,
        splitter,
    })
}

pub fn read_define_scene_and_frame_label_data_tag<R: Read>(
    reader: &mut R,
) -> Result<DefineSceneAndFrameLabelDataTag> {
    let scene_count = reader.read_encoded_u32()?;
    let mut scenes = Vec::with_capacity(scene_count as usize);
    for _ in 0..scene_count {
        scenes.push(read_scene_record(reader)?);
    }
    let frame_label_count = reader.read_encoded_u32()?;
    let mut frame_labels = Vec::with_capacity(frame_label_count as usize);
    for _ in 0..frame_label_count {
        frame_labels.push(read_frame_label_record(reader)?);
    }
    Ok(DefineSceneAndFrameLabelDataTag {
        scenes,
        frame_labels,
    })
}

fn read_scene_record<R: Read>(reader: &mut R) -> Result<SceneRecord> {
    let offset = reader.read_encoded_u32()?;
    let name = String::read(reader)?;
    Ok(SceneRecord { offset, name })
}

fn read_frame_label_record<R: Read>(reader: &mut R) -> Result<FrameLabelRecord> {
    let frame_num = reader.read_encoded_u32()?;
    let frame_label = String::read(reader)?;
    Ok(FrameLabelRecord {
        frame_num,
        frame_label,
    })
}
