use crate::decode::tags::display_list::clip_action_record::ClipActionRecord;
use crate::decode::tags::display_list::ClipEventFlags;

/// Defines one or more event handlers to be invoked when their
/// corresponding events occur.
#[derive(Clone, PartialEq, Debug)]
pub struct ClipActions {
    pub all_event_flags: ClipEventFlags,
    pub clip_action_records: Vec<ClipActionRecord>,
}
