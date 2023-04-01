use crate::decode::tags::control::SymbolClassRecord;

/// Creates associations between characters in the SWF file and ActionScript 3
/// classes.
#[derive(Clone, PartialEq, Debug)]
pub struct SymbolClassTag {
    pub records: Vec<SymbolClassRecord>,
}
