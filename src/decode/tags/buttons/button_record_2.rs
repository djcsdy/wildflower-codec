use crate::decode::bit_read::BitRead;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::buttons::button_record::ButtonRecord;
use crate::decode::tags::buttons::button_record_flags::ButtonRecordFlags;
use crate::decode::tags::common::color_transform_with_alpha::ColorTransformWithAlpha;
use crate::decode::tags::display_list::blend_mode::BlendMode;
use crate::decode::tags::display_list::filter::Filter;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct ButtonRecord2 {
    pub button_record: ButtonRecord,
    pub color_transform: ColorTransformWithAlpha,
    pub filter_list: Vec<Filter>,
    pub blend_mode: BlendMode,
}

impl ButtonRecord2 {
    pub fn read<R: BitRead>(reader: &mut R) -> Result<Option<Self>> {
        if let Some(button_record) = ButtonRecord::read(reader)? {
            let color_transform = ColorTransformWithAlpha::read(reader)?;
            let filter_list = if button_record
                .flags
                .contains(ButtonRecordFlags::HAS_FILTER_LIST)
            {
                Filter::read_list(reader)?
            } else {
                vec![]
            };
            let blend_mode = if button_record
                .flags
                .contains(ButtonRecordFlags::HAS_BLEND_MODE)
            {
                BlendMode::read(reader)?
            } else {
                BlendMode::Normal
            };
            Ok(Some(Self {
                button_record,
                color_transform,
                filter_list,
                blend_mode,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn read_list(reader: &mut SwfSliceReader) -> Result<Vec<Self>> {
        let mut list = Vec::new();
        while let Some(record) = ButtonRecord2::read(reader)? {
            list.push(record);
        }
        Ok(list)
    }
}
