use super::actions::{DoAbcTag, DoActionTag, DoInitActionTag};
use super::bitmaps::{
    DefineBitsJpeg2Tag, DefineBitsJpeg3Tag, DefineBitsJpeg4Tag, DefineBitsLossless2Tag,
    DefineBitsLosslessTag, DefineBitsTag, JpegTablesTag,
};
use super::buttons::{
    DefineButton2Tag, DefineButtonColorTransformTag, DefineButtonSoundTag, DefineButtonTag,
};
use super::control::{
    DefineScalingGridTag, DefineSceneAndFrameLabelDataTag, EnableDebugger2Tag, EnableDebuggerTag,
    EndTag, ExportAssetsTag, FrameLabelTag, ImportAssets2Tag, ImportAssetsTag, MetadataTag,
    ProtectTag, ScriptLimitsTag, SetBackgroundColorTag, SetTabIndexTag, SymbolClassTag,
};
use super::display_list::{
    PlaceObject2Tag, PlaceObject3Tag, PlaceObjectTag, RemoveObject2Tag, RemoveObjectTag,
    ShowFrameTag,
};
use super::fonts::{
    DefineFont2Tag, DefineFont3Tag, DefineFont4Tag, DefineFontAlignZonesTag, DefineFontInfo2Tag,
    DefineFontInfoTag, DefineFontNameTag, DefineFontTag,
};
use super::metadata::{DefineBinaryDataTag, EnableTelemetryTag, FileAttributesTag};
use super::shape_morphing::{DefineMorphShape2Tag, DefineMorphShapeTag};
use super::shapes::{DefineShape2Tag, DefineShape3Tag, DefineShape4Tag, DefineShapeTag};
use super::sounds::{
    DefineSoundTag, SoundStreamBlockTag, SoundStreamHead2Tag, SoundStreamHeadTag, StartSound2Tag,
    StartSoundTag,
};
use super::sprites::DefineSpriteTag;
use super::text::{CsmTextSettingsTag, DefineEditTextTag, DefineText2Tag, DefineTextTag};
use super::video::DefineVideoStreamTag;
use crate::ast::invalid::{InvalidTag, UnknownTag};

#[derive(Clone, PartialEq, Debug)]
pub enum Tag {
    PlaceObject(PlaceObjectTag),
    PlaceObject2(PlaceObject2Tag),
    PlaceObject3(PlaceObject3Tag),
    RemoveObject(RemoveObjectTag),
    RemoveObject2(RemoveObject2Tag),
    ShowFrame(ShowFrameTag),
    SetBackgroundColor(SetBackgroundColorTag),
    FrameLabel(FrameLabelTag),
    Protect(ProtectTag),
    End(EndTag),
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
