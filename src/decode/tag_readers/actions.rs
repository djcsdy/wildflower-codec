use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::actions::action_list::ActionList;
use crate::decode::tags::actions::do_init_action::DoInitActionTag;
use std::io::Result;

pub fn read_do_init_action_tag(reader: &mut SwfSliceReader) -> Result<DoInitActionTag> {
    let sprite_id = reader.read_u16()?;
    let actions = ActionList::read_to_end(reader)?;
    Ok(DoInitActionTag { sprite_id, actions })
}
