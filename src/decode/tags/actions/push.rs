use crate::decode::tags::actions::PushValue;

#[derive(Clone, PartialEq, Debug)]
pub struct Push {
    pub value: PushValue,
}
