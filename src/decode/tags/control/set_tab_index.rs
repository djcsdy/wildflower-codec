/// Sets the tab ordering of the character at the specified depth.
#[derive(Clone, PartialEq, Debug)]
pub struct SetTabIndexTag {
    pub depth: u16,
    pub tab_index: u16,
}
