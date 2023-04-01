use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::common::rectangle::Rectangle;
use crate::decode::tags::common::string::String;
use crate::decode::tags::control::{
    DefineScalingGridTag, DefineSceneAndFrameLabelDataTag, EnableDebugger2Tag, EnableDebuggerTag,
    ExportAssetsTag, FrameLabelRecord, FrameLabelTag, ImportAssets2Tag, ImportAssetsTag,
    MetadataTag, PortableCharacterRecord, SceneRecord, ScriptLimitsTag, SetTabIndexTag,
    SymbolClassRecord, SymbolClassTag,
};
use crate::decode::tags::metadata::{FileAttributesFlags, FileAttributesTag};
use std::io::{Read, Result};

pub fn read_frame_label_tag(reader: &mut SwfSliceReader) -> Result<FrameLabelTag> {
    let name = String::read(reader)?;
    let named_anchor = reader.bytes_remaining() > 0 && reader.read_u8()? == 1;
    Ok(FrameLabelTag { name, named_anchor })
}

pub fn read_export_assets_tag<R: Read>(reader: &mut R) -> Result<ExportAssetsTag> {
    let count = reader.read_u16()?;
    let mut exports = Vec::with_capacity(count as usize);
    for _ in 0..count {
        exports.push(read_portable_character_record(reader)?);
    }
    Ok(ExportAssetsTag { exports })
}

pub fn read_import_assets_tag<R: Read>(reader: &mut R) -> Result<ImportAssetsTag> {
    let url = String::read(reader)?;
    let count = reader.read_u16()?;
    let mut imports = Vec::with_capacity(count as usize);
    for _ in 0..count {
        imports.push(read_portable_character_record(reader)?);
    }
    Ok(ImportAssetsTag { url, imports })
}

fn read_portable_character_record<R: Read>(reader: &mut R) -> Result<PortableCharacterRecord> {
    let character_id = reader.read_u16()?;
    let name = String::read(reader)?;
    Ok(PortableCharacterRecord { character_id, name })
}

pub fn read_enable_debugger_tag<R: Read>(reader: &mut R) -> Result<EnableDebuggerTag> {
    let password_md5 = reader.read_u8_until_null()?;
    Ok(EnableDebuggerTag { password_md5 })
}

pub fn read_enable_debugger_2_tag<R: Read>(reader: &mut R) -> Result<EnableDebugger2Tag> {
    reader.read_u16()?;
    let password_md5 = reader.read_u8_until_null()?;
    Ok(EnableDebugger2Tag { password_md5 })
}

pub fn read_script_limits_tag<R: Read>(reader: &mut R) -> Result<ScriptLimitsTag> {
    let max_recursion_depth = reader.read_u16()?;
    let script_timeout_seconds = reader.read_u16()?;
    Ok(ScriptLimitsTag {
        max_recursion_depth,
        script_timeout_seconds,
    })
}

pub fn read_set_tab_index_tag<R: Read>(reader: &mut R) -> Result<SetTabIndexTag> {
    let depth = reader.read_u16()?;
    let tab_index = reader.read_u16()?;
    Ok(SetTabIndexTag { depth, tab_index })
}

pub fn read_file_attributes_tag<R: Read>(reader: &mut R) -> Result<FileAttributesTag> {
    let flags = FileAttributesFlags::from_bits_truncate(reader.read_u32()?);
    Ok(FileAttributesTag { flags })
}

pub fn read_import_assets_2_tag<R: Read>(reader: &mut R) -> Result<ImportAssets2Tag> {
    let url = String::read(reader)?;
    reader.read_u16()?;
    let count = reader.read_u16()?;
    let mut imports = Vec::with_capacity(count as usize);
    for _ in 0..count {
        imports.push(read_portable_character_record(reader)?);
    }
    Ok(ImportAssets2Tag { url, imports })
}

pub fn read_symbol_class_tag(reader: &mut SwfSliceReader) -> Result<SymbolClassTag> {
    let num_symbols = reader.read_u16()?;
    let mut records = Vec::with_capacity(num_symbols as usize);
    for _ in 0..num_symbols {
        records.push(read_symbol_class_record(reader)?);
    }
    Ok(SymbolClassTag { records })
}

fn read_symbol_class_record(reader: &mut SwfSliceReader) -> Result<SymbolClassRecord> {
    let character_id = reader.read_u16()?;
    let class_name = String::read(reader)?;
    Ok(SymbolClassRecord {
        character_id,
        class_name,
    })
}

pub fn read_metadata_tag(reader: &mut SwfSliceReader) -> Result<MetadataTag> {
    let metadata = String::read(reader)?;
    Ok(MetadataTag { metadata })
}

pub fn read_define_scaling_grid_tag(reader: &mut SwfSliceReader) -> Result<DefineScalingGridTag> {
    let character_id = reader.read_u16()?;
    let splitter = Rectangle::read(reader)?;
    Ok(DefineScalingGridTag {
        character_id,
        splitter,
    })
}

pub fn read_define_scene_and_frame_label_data_tag(
    reader: &mut SwfSliceReader,
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

fn read_scene_record(reader: &mut SwfSliceReader) -> Result<SceneRecord> {
    let offset = reader.read_encoded_u32()?;
    let name = String::read(reader)?;
    Ok(SceneRecord { offset, name })
}

fn read_frame_label_record(reader: &mut SwfSliceReader) -> Result<FrameLabelRecord> {
    let frame_num = reader.read_encoded_u32()?;
    let frame_label = String::read(reader)?;
    Ok(FrameLabelRecord {
        frame_num,
        frame_label,
    })
}
