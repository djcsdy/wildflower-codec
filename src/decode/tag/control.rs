use crate::ast::control::{
    ExportAssetsTag, FrameLabelTag, ImportAssetsTag, PortableCharacterRecord, ProtectTag,
    SetBackgroundColorTag,
};
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::{Read, Result};

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
    let mut imports = Vec::with_capacity(count);
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
