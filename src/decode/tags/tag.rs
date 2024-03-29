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
use crate::decode::tags::control::protect::ProtectTag;
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
use crate::decode::tags::metadata::define_binary_data::DefineBinaryDataTag;
use crate::decode::tags::metadata::enable_telemetry::EnableTelemetryTag;
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
use crate::decode::tags::sounds::start_sound::StartSoundTag;
use crate::decode::tags::sounds::start_sound_2::StartSound2Tag;
use crate::decode::tags::sprites::define_sprite::DefineSpriteTag;
use crate::decode::tags::text::csm_text_settings::CsmTextSettingsTag;
use crate::decode::tags::text::define_edit_text::DefineEditTextTag;
use crate::decode::tags::text::define_text::DefineTextTag;
use crate::decode::tags::text::define_text_2::DefineText2Tag;
use crate::decode::tags::video::define_video_stream::DefineVideoStreamTag;

#[derive(Clone, PartialEq, Debug)]
pub enum Tag {
    PlaceObject(PlaceObjectTag),
    PlaceObject2(PlaceObject2Tag),
    PlaceObject3(PlaceObject3Tag),
    RemoveObject(RemoveObjectTag),
    RemoveObject2(RemoveObject2Tag),
    /// Instructs the player to display the contents of the display list.
    ///
    /// The player will wait at least the duration of one frame before the next
    /// ShowFrame tag takes effect.
    ShowFrame,
    SetBackgroundColor(SetBackgroundColorTag),
    FrameLabel(FrameLabelTag),
    Protect(ProtectTag),
    /// Marks the end of a SWF file.
    End,
    ExportAssets(ExportAssetsTag),
    ImportAssets(ImportAssetsTag),
    EnableDebugger(EnableDebuggerTag),
    EnableDebugger2(EnableDebugger2Tag),
    ScriptLimits(ScriptLimitsTag),
    SetTabIndex(SetTabIndexTag),
    FileAttributes(FileAttributesTag),
    ImportAssets2(ImportAssets2Tag),
    SymbolClass(SymbolClassTag),
    Metadata(MetadataTag),
    DefineScalingGrid(DefineScalingGridTag),
    DefineSceneAndFrameLabelData(DefineSceneAndFrameLabelDataTag),
    DoAction(DoActionTag),
    DoInitAction(DoInitActionTag),
    DoAbc(DoAbcTag),
    DefineShape(DefineShapeTag),
    DefineShape2(DefineShape2Tag),
    DefineShape3(DefineShape3Tag),
    DefineShape4(DefineShape4Tag),
    DefineBits(DefineBitsTag),
    JpegTables(JpegTablesTag),
    DefineBitsJpeg2(DefineBitsJpeg2Tag),
    DefineBitsJpeg3(DefineBitsJpeg3Tag),
    DefineBitsLossless(DefineBitsLosslessTag),
    DefineBitsLossless2(DefineBitsLossless2Tag),
    DefineBitsJpeg4(DefineBitsJpeg4Tag),
    DefineMorphShape(DefineMorphShapeTag),
    DefineMorphShape2(DefineMorphShape2Tag),
    DefineFont(DefineFontTag),
    DefineFontInfo(DefineFontInfoTag),
    DefineFontInfo2(DefineFontInfo2Tag),
    DefineFont2(DefineFont2Tag),
    DefineFont3(DefineFont3Tag),
    DefineFontAlignZones(DefineFontAlignZonesTag),
    DefineFontName(DefineFontNameTag),
    DefineText(DefineTextTag),
    DefineText2(DefineText2Tag),
    DefineEditText(DefineEditTextTag),
    CsmTextSettings(CsmTextSettingsTag),
    DefineFont4(DefineFont4Tag),
    DefineSound(DefineSoundTag),
    StartSound(StartSoundTag),
    StartSound2(StartSound2Tag),
    SoundStreamHead(SoundStreamHeadTag),
    SoundStreamHead2(SoundStreamHead2Tag),
    SoundStreamBlock(SoundStreamBlockTag),
    DefineButton(DefineButtonTag),
    DefineButton2(DefineButton2Tag),
    DefineButtonColorTransform(DefineButtonColorTransformTag),
    DefineButtonSound(DefineButtonSoundTag),
    DefineSprite(DefineSpriteTag),
    DefineVideoStream(DefineVideoStreamTag),
    EnableTelemetry(EnableTelemetryTag),
    DefineBinaryData(DefineBinaryDataTag),
    Unknown(UnknownTag),
    Invalid(InvalidTag),
}
