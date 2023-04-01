pub mod enable_debugger;
pub mod enable_debugger_2;
pub mod export_assets;
pub mod frame_label;
pub mod import_assets;
pub mod portable_character_record;
pub mod protect;
pub mod script_limits;
pub mod set_background_color;
pub mod set_tab_index;

use crate::decode::tags::common::rectangle::Rectangle;
use crate::decode::tags::common::string::String;
use portable_character_record::PortableCharacterRecord;

#[derive(Clone, PartialEq, Debug)]
pub struct ImportAssets2Tag {
    pub url: String,
    pub imports: Vec<PortableCharacterRecord>,
}

/// Creates associations between characters in the SWF file and ActionScript 3
/// classes.
#[derive(Clone, PartialEq, Debug)]
pub struct SymbolClassTag {
    pub records: Vec<SymbolClassRecord>,
}

/// Associates an ActionScript 3 class with a character.
#[derive(Clone, PartialEq, Debug)]
pub struct SymbolClassRecord {
    /// The character ID to be associated.
    pub character_id: u16,

    /// The fully-qualified name of the ActionScript 3 class to be associated.
    pub class_name: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct MetadataTag {
    pub metadata: String,
}

/// Defines a 9-slice grid that should be applied when scaling the specified
/// character.
#[derive(Clone, PartialEq, Debug)]
pub struct DefineScalingGridTag {
    pub character_id: u16,
    pub splitter: Rectangle,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DefineSceneAndFrameLabelDataTag {
    pub scenes: Vec<SceneRecord>,
    pub frame_labels: Vec<FrameLabelRecord>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SceneRecord {
    pub offset: u32,
    pub name: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct FrameLabelRecord {
    pub frame_num: u32,
    pub frame_label: String,
}
