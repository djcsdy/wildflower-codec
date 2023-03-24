use crate::decode::tags::actions::ActionRecord;

#[derive(Clone, PartialEq, Debug)]
pub struct With {
    pub body: Vec<ActionRecord>,
}
