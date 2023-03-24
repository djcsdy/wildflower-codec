use crate::decode::tags::actions::push_value::PushValue;

#[derive(Clone, PartialEq, Debug)]
pub struct Push {
    pub value: PushValue,
}
