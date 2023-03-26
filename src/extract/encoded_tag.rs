use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tag_readers::bitmaps::{
    read_define_bits_jpeg_2_tag, read_define_bits_jpeg_3_tag, read_define_bits_jpeg_4_tag,
    read_define_bits_lossless_2_tag, read_define_bits_lossless_tag, read_define_bits_tag,
    read_jpeg_tables_tag,
};
use crate::decode::tag_readers::control::{
    read_define_scaling_grid_tag, read_define_scene_and_frame_label_data_tag,
    read_enable_debugger_2_tag, read_enable_debugger_tag, read_export_assets_tag,
    read_file_attributes_tag, read_frame_label_tag, read_import_assets_2_tag,
    read_import_assets_tag, read_metadata_tag, read_script_limits_tag,
    read_set_background_color_tag, read_set_tab_index_tag, read_symbol_class_tag,
};
use crate::decode::tag_readers::display_list::{
    read_place_object_2_tag, read_place_object_3_tag, read_place_object_tag,
    read_remove_object_2_tag, read_remove_object_tag,
};
use crate::decode::tag_readers::shape_morphing::{
    read_define_morph_shape_2_tag, read_define_morph_shape_tag,
};
use crate::decode::tag_readers::shapes::{
    read_define_shape_2_tag, read_define_shape_3_tag, read_define_shape_4_tag,
    read_define_shape_tag,
};
use crate::decode::tags::actions::do_abc::DoAbcTag;
use crate::decode::tags::actions::do_action::DoActionTag;
use crate::decode::tags::actions::do_init_action::DoInitActionTag;
use crate::decode::tags::buttons::define_button::DefineButtonTag;
use crate::decode::tags::fonts::define_font::DefineFontTag;
use crate::decode::tags::fonts::define_font_2::DefineFont2Tag;
use crate::decode::tags::fonts::define_font_info::DefineFontInfoTag;
use crate::decode::tags::fonts::define_font_info_2::DefineFontInfo2Tag;
use crate::decode::tags::invalid::{InvalidTag, UnknownTag};
use crate::decode::tags::tag::Tag;
use crate::extract::tag_type::TagType;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct EncodedTag {
    swf_version: u8,
    tag_type: TagType,
    body: Vec<u8>,
}

impl EncodedTag {
    pub fn new(swf_version: u8, tag_type: TagType, body: Vec<u8>) -> EncodedTag {
        EncodedTag {
            swf_version,
            tag_type,
            body,
        }
    }

    pub fn decode(self) -> Tag {
        let mut slice_reader = SwfSliceReader::new(&self.body, self.swf_version);
        match self.tag_type {
            TagType::End => Tag::Unknown(self.into_unknown()),
            TagType::ShowFrame => Tag::Unknown(self.into_unknown()),
            TagType::DefineShape => read_define_shape_tag(&mut slice_reader)
                .map(Tag::DefineShape)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::PlaceObject => read_place_object_tag(&mut slice_reader)
                .map(Tag::PlaceObject)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::RemoveObject => read_remove_object_tag(&mut slice_reader)
                .map(Tag::RemoveObject)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineBits => read_define_bits_tag(&mut slice_reader)
                .map(Tag::DefineBits)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineButton => DefineButtonTag::read(&mut slice_reader)
                .map(Tag::DefineButton)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::JpegTables => read_jpeg_tables_tag(&mut slice_reader)
                .map(Tag::JpegTables)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::SetBackgroundColor => read_set_background_color_tag(&mut slice_reader)
                .map(Tag::SetBackgroundColor)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineFont => DefineFontTag::read(&mut slice_reader)
                .map(Tag::DefineFont)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineText => Tag::Unknown(self.into_unknown()),
            TagType::DoAction => DoActionTag::read(&mut slice_reader)
                .map(Tag::DoAction)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineFontInfo => DefineFontInfoTag::read(&mut slice_reader)
                .map(Tag::DefineFontInfo)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineSound => Tag::Unknown(self.into_unknown()),
            TagType::StartSound => Tag::Unknown(self.into_unknown()),
            TagType::DefineButtonSound => Tag::Unknown(self.into_unknown()),
            TagType::SoundStreamHead => Tag::Unknown(self.into_unknown()),
            TagType::SoundStreamBlock => Tag::Unknown(self.into_unknown()),
            TagType::DefineBitsLossless => read_define_bits_lossless_tag(&mut slice_reader)
                .map(Tag::DefineBitsLossless)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineBitsJpeg2 => read_define_bits_jpeg_2_tag(&mut slice_reader)
                .map(Tag::DefineBitsJpeg2)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineShape2 => read_define_shape_2_tag(&mut slice_reader)
                .map(Tag::DefineShape2)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineButtonColorTransform => Tag::Unknown(self.into_unknown()),
            TagType::Protect => Tag::Unknown(self.into_unknown()),
            TagType::PlaceObject2 => read_place_object_2_tag(&mut slice_reader)
                .map(Tag::PlaceObject2)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::RemoveObject2 => read_remove_object_2_tag(&mut slice_reader)
                .map(Tag::RemoveObject2)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineShape3 => read_define_shape_3_tag(&mut slice_reader)
                .map(Tag::DefineShape3)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineText2 => Tag::Unknown(self.into_unknown()),
            TagType::DefineButton2 => Tag::Unknown(self.into_unknown()),
            TagType::DefineBitsJpeg3 => read_define_bits_jpeg_3_tag(&mut slice_reader)
                .map(Tag::DefineBitsJpeg3)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineBitsLossless2 => read_define_bits_lossless_2_tag(&mut slice_reader)
                .map(Tag::DefineBitsLossless2)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineEditText => Tag::Unknown(self.into_unknown()),
            TagType::DefineSprite => Tag::Unknown(self.into_unknown()),
            TagType::FrameLabel => read_frame_label_tag(&mut slice_reader)
                .map(Tag::FrameLabel)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::SoundStreamHead2 => Tag::Unknown(self.into_unknown()),
            TagType::DefineMorphShape => read_define_morph_shape_tag(&mut slice_reader)
                .map(Tag::DefineMorphShape)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineFont2 => DefineFont2Tag::read(&mut slice_reader)
                .map(Tag::DefineFont2)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::ExportAssets => read_export_assets_tag(&mut slice_reader)
                .map(Tag::ExportAssets)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::ImportAssets => read_import_assets_tag(&mut slice_reader)
                .map(Tag::ImportAssets)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::EnableDebugger => read_enable_debugger_tag(&mut slice_reader)
                .map(Tag::EnableDebugger)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DoInitAction => DoInitActionTag::read(&mut slice_reader)
                .map(Tag::DoInitAction)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineVideoStream => Tag::Unknown(self.into_unknown()),
            TagType::VideoFrame => Tag::Unknown(self.into_unknown()),
            TagType::DefineFontInfo2 => DefineFontInfo2Tag::read(&mut slice_reader)
                .map(Tag::DefineFontInfo2)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::EnableDebugger2 => read_enable_debugger_2_tag(&mut slice_reader)
                .map(Tag::EnableDebugger2)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::ScriptLimits => read_script_limits_tag(&mut slice_reader)
                .map(Tag::ScriptLimits)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::SetTabIndex => read_set_tab_index_tag(&mut slice_reader)
                .map(Tag::SetTabIndex)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::FileAttributes => read_file_attributes_tag(&mut slice_reader)
                .map(Tag::FileAttributes)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::PlaceObject3 => read_place_object_3_tag(&mut slice_reader)
                .map(Tag::PlaceObject3)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::ImportAssets2 => read_import_assets_2_tag(&mut slice_reader)
                .map(Tag::ImportAssets2)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineFontAlignZones => Tag::Unknown(self.into_unknown()),
            TagType::CsmTextSettings => Tag::Unknown(self.into_unknown()),
            TagType::DefineFont3 => Tag::Unknown(self.into_unknown()),
            TagType::SymbolClass => read_symbol_class_tag(&mut slice_reader)
                .map(Tag::SymbolClass)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::Metadata => read_metadata_tag(&mut slice_reader)
                .map(Tag::Metadata)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineScalingGrid => read_define_scaling_grid_tag(&mut slice_reader)
                .map(Tag::DefineScalingGrid)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DoAbc => DoAbcTag::read(&mut slice_reader)
                .map(Tag::DoAbc)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineShape4 => read_define_shape_4_tag(&mut slice_reader)
                .map(Tag::DefineShape4)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineMorphShape2 => read_define_morph_shape_2_tag(&mut slice_reader)
                .map(Tag::DefineMorphShape2)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineSceneAndFrameLabelData => {
                read_define_scene_and_frame_label_data_tag(&mut slice_reader)
                    .map(Tag::DefineSceneAndFrameLabelData)
                    .unwrap_or_else(|_| Tag::Invalid(self.into_invalid()))
            }
            TagType::DefineBinaryData => Tag::Unknown(self.into_unknown()),
            TagType::DefineFontName => Tag::Unknown(self.into_unknown()),
            TagType::StartSound2 => Tag::Unknown(self.into_unknown()),
            TagType::DefineBitsJpeg4 => read_define_bits_jpeg_4_tag(&mut slice_reader)
                .map(Tag::DefineBitsJpeg4)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineFont4 => Tag::Unknown(self.into_unknown()),
            TagType::EnableTelemetry => Tag::Unknown(self.into_unknown()),
            TagType::Unknown(_) => Tag::Unknown(self.into_unknown()),
        }
    }

    fn into_unknown(self) -> UnknownTag {
        UnknownTag {
            tag_code: self.tag_type.into(),
            body: self.body,
        }
    }

    fn into_invalid(self) -> InvalidTag {
        InvalidTag {
            tag_code: self.tag_type.into(),
            body: self.body,
        }
    }
}
