use crate::decode::tags::control::scene_record::SceneRecord;
use crate::decode::tags::control::FrameLabelRecord;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineSceneAndFrameLabelDataTag {
    pub scenes: Vec<SceneRecord>,
    pub frame_labels: Vec<FrameLabelRecord>,
}
