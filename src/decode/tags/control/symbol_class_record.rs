use crate::decode::tags::common::string::String;

/// Associates an ActionScript 3 class with a character.
#[derive(Clone, PartialEq, Debug)]
pub struct SymbolClassRecord {
    /// The character ID to be associated.
    pub character_id: u16,

    /// The fully-qualified name of the ActionScript 3 class to be associated.
    pub class_name: String,
}
