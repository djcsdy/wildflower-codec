pub mod define_scaling_grid;
pub mod define_scene_and_frame_label_data;
pub mod enable_debugger;
pub mod enable_debugger_2;
pub mod export_assets;
pub mod frame_label;
pub mod import_assets;
pub mod import_assets_2;
pub mod metadata;
pub mod portable_character_record;
pub mod protect;
pub mod scene_record;
pub mod script_limits;
pub mod set_background_color;
pub mod set_tab_index;
pub mod symbol_class;
pub mod symbol_class_record;

use crate::decode::tags::common::string::String;

#[derive(Clone, PartialEq, Debug)]
pub struct FrameLabelRecord {
    pub frame_num: u32,
    pub frame_label: String,
}
