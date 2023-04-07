use crate::decode::tags::actions::action_list::ActionList;
use crate::decode::tags::display_list::ClipEventFlags;

#[derive(Clone, PartialEq, Debug)]
pub struct ClipActionRecord {
    pub event_flags: ClipEventFlags,
    pub key_code: Option<u8>,
    pub actions: ActionList<Vec<u8>>,
}
