use crate::decode::read_options::ReadOptions;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::swf_version::SwfVersion;
use crate::decode::tags::actions::do_abc::DoAbcTag;
use crate::decode::tags::actions::do_action::DoActionTag;
use crate::decode::tags::actions::do_init_action::DoInitActionTag;
use crate::decode::tags::bitmaps::define_bits::DefineBitsTag;
use crate::decode::tags::bitmaps::define_bits_jpeg_2::DefineBitsJpeg2Tag;
use crate::decode::tags::bitmaps::define_bits_jpeg_3::DefineBitsJpeg3Tag;
use crate::decode::tags::bitmaps::define_bits_jpeg_4::DefineBitsJpeg4Tag;
use crate::decode::tags::bitmaps::define_bits_lossless::DefineBitsLosslessTag;
use crate::decode::tags::bitmaps::define_bits_lossless_2::DefineBitsLossless2Tag;
use crate::decode::tags::bitmaps::jpeg_tables::JpegTablesTag;
use crate::decode::tags::buttons::define_button::DefineButtonTag;
use crate::decode::tags::buttons::define_button_2::DefineButton2Tag;
use crate::decode::tags::buttons::define_button_color_transform::DefineButtonColorTransformTag;
use crate::decode::tags::buttons::define_button_sound::DefineButtonSoundTag;
use crate::decode::tags::control::define_scaling_grid::DefineScalingGridTag;
use crate::decode::tags::control::define_scene_and_frame_label_data::DefineSceneAndFrameLabelDataTag;
use crate::decode::tags::control::enable_debugger::EnableDebuggerTag;
use crate::decode::tags::control::enable_debugger_2::EnableDebugger2Tag;
use crate::decode::tags::control::export_assets::ExportAssetsTag;
use crate::decode::tags::control::frame_label::FrameLabelTag;
use crate::decode::tags::control::import_assets::ImportAssetsTag;
use crate::decode::tags::control::import_assets_2::ImportAssets2Tag;
use crate::decode::tags::control::metadata::MetadataTag;
use crate::decode::tags::control::script_limits::ScriptLimitsTag;
use crate::decode::tags::control::set_background_color::SetBackgroundColorTag;
use crate::decode::tags::control::set_tab_index::SetTabIndexTag;
use crate::decode::tags::control::symbol_class::SymbolClassTag;
use crate::decode::tags::display_list::place_object::PlaceObjectTag;
use crate::decode::tags::display_list::place_object_2::PlaceObject2Tag;
use crate::decode::tags::display_list::place_object_3::PlaceObject3Tag;
use crate::decode::tags::display_list::remove_object::RemoveObjectTag;
use crate::decode::tags::display_list::remove_object_2::RemoveObject2Tag;
use crate::decode::tags::fonts::define_font::DefineFontTag;
use crate::decode::tags::fonts::define_font_2::DefineFont2Tag;
use crate::decode::tags::fonts::define_font_3::DefineFont3Tag;
use crate::decode::tags::fonts::define_font_4::DefineFont4Tag;
use crate::decode::tags::fonts::define_font_align_zones::DefineFontAlignZonesTag;
use crate::decode::tags::fonts::define_font_info::DefineFontInfoTag;
use crate::decode::tags::fonts::define_font_info_2::DefineFontInfo2Tag;
use crate::decode::tags::fonts::define_font_name::DefineFontNameTag;
use crate::decode::tags::invalid::{InvalidTag, UnknownTag};
use crate::decode::tags::metadata::file_attributes::FileAttributesTag;
use crate::decode::tags::shape_morphing::define_morph_shape::DefineMorphShapeTag;
use crate::decode::tags::shape_morphing::define_morph_shape_2::DefineMorphShape2Tag;
use crate::decode::tags::shapes::define_shape::DefineShapeTag;
use crate::decode::tags::shapes::define_shape_2::DefineShape2Tag;
use crate::decode::tags::shapes::define_shape_3::DefineShape3Tag;
use crate::decode::tags::shapes::define_shape_4::DefineShape4Tag;
use crate::decode::tags::sounds::define_sound::DefineSoundTag;
use crate::decode::tags::sounds::sound_stream_block::SoundStreamBlockTag;
use crate::decode::tags::sounds::sound_stream_head::SoundStreamHeadTag;
use crate::decode::tags::sounds::sound_stream_head_2::SoundStreamHead2Tag;
use crate::decode::tags::sounds::start_sound_2::StartSound2Tag;
use crate::decode::tags::sprites::define_sprite::DefineSpriteTag;
use crate::decode::tags::tag::Tag;
use crate::decode::tags::text::csm_text_settings::CsmTextSettingsTag;
use crate::decode::tags::text::define_edit_text::DefineEditTextTag;
use crate::decode::tags::text::define_text::DefineTextTag;
use crate::decode::tags::text::define_text_2::DefineText2Tag;
use crate::extract::tag_type::TagType;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct EncodedTag {
    swf_version: SwfVersion,
    tag_type: TagType,
    body: Vec<u8>,
}

impl EncodedTag {
    pub fn new(swf_version: SwfVersion, tag_type: TagType, body: Vec<u8>) -> EncodedTag {
        EncodedTag {
            swf_version,
            tag_type,
            body,
        }
    }

    pub fn decode(self) -> Tag {
        let mut slice_reader = SwfSliceReader::new(&self.body);
        match self.tag_type {
            TagType::End => Tag::End,
            TagType::ShowFrame => Tag::ShowFrame,
            TagType::DefineShape => DefineShapeTag::read(&mut slice_reader)
                .map(Tag::DefineShape)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::PlaceObject => PlaceObjectTag::read(&mut slice_reader)
                .map(Tag::PlaceObject)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::RemoveObject => RemoveObjectTag::read(&mut slice_reader)
                .map(Tag::RemoveObject)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineBits => DefineBitsTag::read(&mut slice_reader)
                .map(Tag::DefineBits)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineButton => DefineButtonTag::read(&mut slice_reader)
                .map(Tag::DefineButton)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::JpegTables => JpegTablesTag::read(&mut slice_reader)
                .map(Tag::JpegTables)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::SetBackgroundColor => SetBackgroundColorTag::read(&mut slice_reader)
                .map(Tag::SetBackgroundColor)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineFont => DefineFontTag::read(&mut slice_reader)
                .map(Tag::DefineFont)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineText => DefineTextTag::read(&mut slice_reader)
                .map(Tag::DefineText)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DoAction => DoActionTag::read(&mut slice_reader)
                .map(Tag::DoAction)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineFontInfo => DefineFontInfoTag::read(&mut slice_reader)
                .map(Tag::DefineFontInfo)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineSound => DefineSoundTag::read(&mut slice_reader)
                .map(Tag::DefineSound)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::StartSound => Tag::Unknown(self.into_unknown()),
            TagType::DefineButtonSound => DefineButtonSoundTag::read(&mut slice_reader)
                .map(Tag::DefineButtonSound)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::SoundStreamHead => SoundStreamHeadTag::read(&mut slice_reader)
                .map(Tag::SoundStreamHead)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::SoundStreamBlock => SoundStreamBlockTag::read(&mut slice_reader)
                .map(Tag::SoundStreamBlock)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineBitsLossless => DefineBitsLosslessTag::read(&mut slice_reader)
                .map(Tag::DefineBitsLossless)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineBitsJpeg2 => DefineBitsJpeg2Tag::read(&mut slice_reader)
                .map(Tag::DefineBitsJpeg2)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineShape2 => DefineShape2Tag::read(&mut slice_reader)
                .map(Tag::DefineShape2)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineButtonColorTransform => {
                DefineButtonColorTransformTag::read(&mut slice_reader)
                    .map(Tag::DefineButtonColorTransform)
                    .unwrap_or_else(|_| Tag::Invalid(self.into_invalid()))
            }
            TagType::Protect => Tag::Unknown(self.into_unknown()),
            TagType::PlaceObject2 => PlaceObject2Tag::read(ReadOptions {
                reader: &mut slice_reader,
                swf_version: self.swf_version,
            })
                .map(Tag::PlaceObject2)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::RemoveObject2 => RemoveObject2Tag::read(&mut slice_reader)
                .map(Tag::RemoveObject2)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineShape3 => DefineShape3Tag::read(&mut slice_reader)
                .map(Tag::DefineShape3)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineText2 => DefineText2Tag::read(&mut slice_reader)
                .map(Tag::DefineText2)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineButton2 => DefineButton2Tag::read(&mut slice_reader)
                .map(Tag::DefineButton2)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineBitsJpeg3 => DefineBitsJpeg3Tag::read(&mut slice_reader)
                .map(Tag::DefineBitsJpeg3)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineBitsLossless2 => DefineBitsLossless2Tag::read(&mut slice_reader)
                .map(Tag::DefineBitsLossless2)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineEditText => DefineEditTextTag::read(&mut slice_reader)
                .map(Tag::DefineEditText)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineSprite => DefineSpriteTag::read(&mut slice_reader)
                .map(Tag::DefineSprite)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::FrameLabel => FrameLabelTag::read(&mut slice_reader)
                .map(Tag::FrameLabel)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::SoundStreamHead2 => SoundStreamHead2Tag::read(&mut slice_reader)
                .map(Tag::SoundStreamHead2)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineMorphShape => DefineMorphShapeTag::read(&mut slice_reader)
                .map(Tag::DefineMorphShape)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineFont2 => DefineFont2Tag::read(&mut slice_reader)
                .map(Tag::DefineFont2)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::ExportAssets => ExportAssetsTag::read(&mut slice_reader)
                .map(Tag::ExportAssets)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::ImportAssets => ImportAssetsTag::read(&mut slice_reader)
                .map(Tag::ImportAssets)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::EnableDebugger => EnableDebuggerTag::read(&mut slice_reader)
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
            TagType::EnableDebugger2 => EnableDebugger2Tag::read(&mut slice_reader)
                .map(Tag::EnableDebugger2)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::ScriptLimits => ScriptLimitsTag::read(&mut slice_reader)
                .map(Tag::ScriptLimits)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::SetTabIndex => SetTabIndexTag::read(&mut slice_reader)
                .map(Tag::SetTabIndex)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::FileAttributes => FileAttributesTag::read(&mut slice_reader)
                .map(Tag::FileAttributes)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::PlaceObject3 => PlaceObject3Tag::read(ReadOptions {
                reader: &mut slice_reader,
                swf_version: self.swf_version,
            })
                .map(Tag::PlaceObject3)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::ImportAssets2 => ImportAssets2Tag::read(&mut slice_reader)
                .map(Tag::ImportAssets2)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineFontAlignZones => DefineFontAlignZonesTag::read(&mut slice_reader)
                .map(Tag::DefineFontAlignZones)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::CsmTextSettings => CsmTextSettingsTag::read(&mut slice_reader)
                .map(Tag::CsmTextSettings)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineFont3 => DefineFont3Tag::read(&mut slice_reader)
                .map(Tag::DefineFont3)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::SymbolClass => SymbolClassTag::read(&mut slice_reader)
                .map(Tag::SymbolClass)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::Metadata => MetadataTag::read(&mut slice_reader)
                .map(Tag::Metadata)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineScalingGrid => DefineScalingGridTag::read(&mut slice_reader)
                .map(Tag::DefineScalingGrid)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DoAbc => DoAbcTag::read(&mut slice_reader)
                .map(Tag::DoAbc)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineShape4 => DefineShape4Tag::read(&mut slice_reader)
                .map(Tag::DefineShape4)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineMorphShape2 => DefineMorphShape2Tag::read(&mut slice_reader)
                .map(Tag::DefineMorphShape2)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineSceneAndFrameLabelData => {
                DefineSceneAndFrameLabelDataTag::read(&mut slice_reader)
                    .map(Tag::DefineSceneAndFrameLabelData)
                    .unwrap_or_else(|_| Tag::Invalid(self.into_invalid()))
            }
            TagType::DefineBinaryData => Tag::Unknown(self.into_unknown()),
            TagType::DefineFontName => DefineFontNameTag::read(&mut slice_reader)
                .map(Tag::DefineFontName)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::StartSound2 => StartSound2Tag::read(&mut slice_reader)
                .map(Tag::StartSound2)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineBitsJpeg4 => DefineBitsJpeg4Tag::read(&mut slice_reader)
                .map(Tag::DefineBitsJpeg4)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
            TagType::DefineFont4 => DefineFont4Tag::read(&mut slice_reader)
                .map(Tag::DefineFont4)
                .unwrap_or_else(|_| Tag::Invalid(self.into_invalid())),
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
