use crate::ast::control::{
    DefineScalingGridTag, DefineSceneAndFrameLabelDataTag, EnableDebugger2Tag, EnableDebuggerTag,
    ExportAssetsTag, FileAttributesFlags, FileAttributesTag, FrameLabelRecord, FrameLabelTag,
    ImportAssets2Tag, ImportAssetsTag, MetadataTag, PortableCharacterRecord, ProtectTag,
    SceneRecord, ScriptLimitsTag, SetBackgroundColorTag, SetTabIndexTag, SymbolClassRecord,
    SymbolClassTag,
};
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read, Result};

pub fn read_set_background_color_tag<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<SetBackgroundColorTag> {
    let color = reader.read_rgb()?;
    Ok(SetBackgroundColorTag { color })
}

pub fn read_frame_label_tag<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<FrameLabelTag> {
    let name = reader.read_string()?;
    let named_anchor = reader.remaining() > 0 && reader.read_u8()? == 1;
    Ok(FrameLabelTag { name, named_anchor })
}

pub fn read_export_assets_tag<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<ExportAssetsTag> {
    let count = reader.read_u16()?;
    let mut exports = Vec::with_capacity(count as usize);
    for _ in 0..count {
        exports.push(read_portable_character_record(reader)?);
    }
    Ok(ExportAssetsTag { exports })
}

pub fn read_import_assets_tag<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<ImportAssetsTag> {
    let url = reader.read_string()?;
    let count = reader.read_u16()?;
    let mut imports = Vec::with_capacity(count as usize);
    for _ in 0..count {
        imports.push(read_portable_character_record(reader)?);
    }
    Ok(ImportAssetsTag { url, imports })
}

fn read_portable_character_record<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<PortableCharacterRecord> {
    let character_id = reader.read_u16()?;
    let name = reader.read_string()?;
    Ok(PortableCharacterRecord { character_id, name })
}

pub fn read_enable_debugger_tag<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<EnableDebuggerTag> {
    let password_md5 = reader.read_null_terminated_bytes()?;
    Ok(EnableDebuggerTag { password_md5 })
}

pub fn read_enable_debugger2_tag<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<EnableDebugger2Tag> {
    reader.read_u16()?;
    let password_md5 = reader.read_null_terminated_bytes()?;
    Ok(EnableDebugger2Tag { password_md5 })
}

pub fn read_script_limits_tag<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<ScriptLimitsTag> {
    let max_recursion_depth = reader.read_u16()?;
    let script_timeout_seconds = reader.read_u16()?;
    Ok(ScriptLimitsTag {
        max_recursion_depth,
        script_timeout_seconds,
    })
}

pub fn read_set_tab_index_tag<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<SetTabIndexTag> {
    let depth = reader.read_u16()?;
    let tab_index = reader.read_u16()?;
    Ok(SetTabIndexTag { depth, tab_index })
}

pub fn read_file_attributes_tag<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<FileAttributesTag> {
    let flags = FileAttributesFlags::from_bits_truncate(reader.read_u32()?);
    Ok(FileAttributesTag { flags })
}

pub fn read_import_assets2_tag<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<ImportAssets2Tag> {
    let url = reader.read_string()?;
    reader.read_u16()?;
    let count = reader.read_u16()?;
    let mut imports = Vec::with_capacity(count as usize);
    for _ in 0..count {
        imports.push(read_portable_character_record(reader)?);
    }
    Ok(ImportAssets2Tag { url, imports })
}

pub fn read_symbol_class_tag<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<SymbolClassTag> {
    let num_symbols = reader.read_u16()?;
    let mut records = Vec::with_capacity(num_symbols as usize);
    for _ in 0..num_symbols {
        records.push(read_symbol_class_record(reader)?);
    }
    Ok(SymbolClassTag { records })
}

fn read_symbol_class_record<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<SymbolClassRecord> {
    let character_id = reader.read_u16()?;
    let class_name = reader.read_string()?;
    Ok(SymbolClassRecord {
        character_id,
        class_name,
    })
}

pub fn read_metadata_tag<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<MetadataTag> {
    let metadata = reader.read_string()?;
    Ok(MetadataTag { metadata })
}

pub fn read_define_scaling_grid_tag<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<DefineScalingGridTag> {
    let character_id = reader.read_u16()?;
    let splitter = reader.read_rectangle()?;
    Ok(DefineScalingGridTag {
        character_id,
        splitter,
    })
}

pub fn read_define_scene_and_frame_label_data_tag<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
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

fn read_scene_record<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<SceneRecord> {
    let offset = reader.read_encoded_u32()?;
    let name = reader.read_string()?;
    Ok(SceneRecord { offset, name })
}

fn read_frame_label_record<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<FrameLabelRecord> {
    let frame_num = reader.read_encoded_u32()?;
    let frame_label = reader.read_string()?;
    Ok(FrameLabelRecord {
        frame_num,
        frame_label,
    })
}
