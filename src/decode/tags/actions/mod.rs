pub mod action_list;
pub mod action_offset;
pub mod action_pointer;
pub mod action_record;
pub mod constant_pool;
pub mod define_function;
pub mod define_function_2;
pub mod do_abc;
pub mod do_action;
pub mod get_url;
pub mod get_url_2;
pub mod go_to_frame;
pub mod go_to_frame_2;
pub mod go_to_label;
pub mod r#if;
pub mod jump;
pub mod load_target;
pub mod push;
pub mod push_value;
pub mod register_param;
pub mod send_vars_method;
pub mod set_target;
pub mod store_register;
pub mod r#try;
pub mod wait_for_frame;
pub mod wait_for_frame_2;
pub mod with;

use crate::decode::tags::actions::action_list::ActionList;

#[derive(Clone, PartialEq, Debug)]
pub struct DoInitActionTag {
    pub sprite_id: u16,
    pub actions: ActionList<Vec<u8>>,
}
