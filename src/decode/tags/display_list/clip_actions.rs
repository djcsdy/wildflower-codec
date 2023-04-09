use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::read_options::ReadOptions;
use crate::decode::slice_read::SliceRead;
use crate::decode::tags::display_list::clip_action_record::ClipActionRecord;
use crate::decode::tags::display_list::clip_event_flags::ClipEventFlags;
use std::io::Result;

/// Defines one or more event handlers to be invoked when their
/// corresponding events occur.
#[derive(Clone, PartialEq, Debug)]
pub struct ClipActions {
    pub all_event_flags: ClipEventFlags,
    pub clip_action_records: Vec<ClipActionRecord>,
}

impl ClipActions {
    pub fn read<R: SliceRead>(
        ReadOptions {
            reader,
            swf_version,
        }: ReadOptions<R>,
    ) -> Result<Self> {
        reader.read_u16()?;
        let all_event_flags = ClipEventFlags::read(ReadOptions {
            reader,
            swf_version,
        })?;
        let mut clip_action_records = Vec::new();
        while let Some(clip_action_record) = ClipActionRecord::read(ReadOptions {
            reader,
            swf_version,
        })? {
            clip_action_records.push(clip_action_record);
        }
        Ok(Self {
            all_event_flags,
            clip_action_records,
        })
    }
}
