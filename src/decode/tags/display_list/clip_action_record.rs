use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::read_options::ReadOptions;
use crate::decode::slice_read::SliceRead;
use crate::decode::tags::actions::action_list::ActionList;
use crate::decode::tags::display_list::clip_event_flags::ClipEventFlags;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct ClipActionRecord {
    pub event_flags: ClipEventFlags,
    pub key_code: Option<u8>,
    pub actions: ActionList<Vec<u8>>,
}

impl ClipActionRecord {
    pub fn read<R: SliceRead>(
        ReadOptions {
            reader,
            swf_version,
        }: ReadOptions<R>,
    ) -> Result<Option<Self>> {
        let event_flags = ClipEventFlags::read(ReadOptions {
            reader,
            swf_version,
        })?;
        if event_flags.is_empty() {
            Ok(None)
        } else {
            let action_record_size = reader.read_u32()?;
            let mut action_record_reader = reader.slice(action_record_size as usize);
            let key_code = if event_flags.contains(ClipEventFlags::KEY_PRESS) {
                Some(action_record_reader.read_u8()?)
            } else {
                None
            };
            let actions = ActionList::read_to_end(&mut action_record_reader)?;
            Ok(Some(Self {
                event_flags,
                key_code,
                actions,
            }))
        }
    }
}
