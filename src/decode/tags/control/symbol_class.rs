use crate::decode::tags::control::symbol_class_record::SymbolClassRecord;

/// Creates associations between characters in the SWF file and ActionScript 3
/// classes.
#[derive(Clone, PartialEq, Debug)]
pub struct SymbolClassTag {
    pub records: Vec<SymbolClassRecord>,
}
