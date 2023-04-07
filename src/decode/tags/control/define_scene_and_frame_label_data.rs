use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::control::frame_label_record::FrameLabelRecord;
use crate::decode::tags::control::scene_record::SceneRecord;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct DefineSceneAndFrameLabelDataTag {
    pub scenes: Vec<SceneRecord>,
    pub frame_labels: Vec<FrameLabelRecord>,
}

impl DefineSceneAndFrameLabelDataTag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let scene_count = reader.read_encoded_u32()?;
        let mut scenes = Vec::with_capacity(scene_count as usize);
        for _ in 0..scene_count {
            scenes.push(SceneRecord::read(reader)?);
        }
        let frame_label_count = reader.read_encoded_u32()?;
        let mut frame_labels = Vec::with_capacity(frame_label_count as usize);
        for _ in 0..frame_label_count {
            frame_labels.push(FrameLabelRecord::read(reader)?);
        }
        Ok(Self {
            scenes,
            frame_labels,
        })
    }
}
