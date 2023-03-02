use crate::ast::actions::{DoAbcTag, DoActionTag, DoInitActionTag};
use crate::ast::bitmaps::{
    DefineBitsJpeg2Tag, DefineBitsJpeg3Tag, DefineBitsJpeg4Tag, DefineBitsLossless2Tag,
    DefineBitsLosslessTag, DefineBitsTag, JpegTablesTag,
};
use crate::ast::buttons::{
    DefineButton2Tag, DefineButtonColorTransformTag, DefineButtonSoundTag, DefineButtonTag,
};
use crate::ast::control::{
    DefineScalingGridTag, DefineSceneAndFrameLabelDataTag, EnableDebugger2Tag, EnableDebuggerTag,
    EndTag, ExportAssetsTag, FrameLabelTag, ImportAssets2Tag, ImportAssetsTag, MetadataTag,
    ProtectTag, ScriptLimitsTag, SetBackgroundColorTag, SetTabIndexTag, SymbolClassTag,
};
use crate::ast::display_list::{
    PlaceObject2Tag, PlaceObject3Tag, PlaceObjectTag, RemoveObject2Tag, RemoveObjectTag,
    ShowFrameTag,
};
use crate::ast::fonts::{
    DefineFont2Tag, DefineFont3Tag, DefineFont4Tag, DefineFontAlignZonesTag, DefineFontInfo2Tag,
    DefineFontInfoTag, DefineFontNameTag, DefineFontTag,
};
use crate::ast::metadata::{DefineBinaryDataTag, EnableTelemetryTag, FileAttributesTag};
use crate::ast::shape_morphing::{DefineMorphShape2Tag, DefineMorphShapeTag};
use crate::ast::shapes::{DefineShape2Tag, DefineShape3Tag, DefineShape4Tag, DefineShapeTag};
use crate::ast::sounds::{
    DefineSoundTag, SoundStreamBlockTag, SoundStreamHead2Tag, SoundStreamHeadTag, StartSound2Tag,
    StartSoundTag,
};
use crate::ast::sprites::DefineSpriteTag;
use crate::ast::text::{CsmTextSettingsTag, DefineEditTextTag, DefineText2Tag, DefineTextTag};
use crate::ast::video::DefineVideoStreamTag;

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
}
