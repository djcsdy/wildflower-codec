use crate::decode::tags::control::frame_label_record::FrameLabelRecord;
use crate::decode::tags::control::scene_record::SceneRecord;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineSceneAndFrameLabelDataTag {
    pub scenes: Vec<SceneRecord>,
    pub frame_labels: Vec<FrameLabelRecord>,
}
