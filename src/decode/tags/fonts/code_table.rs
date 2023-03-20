#[derive(Clone, PartialEq, Debug)]
pub enum CodeTable {
    Windows1252(Vec<u8>),
    JisX0201(Vec<u8>),
    ShiftJis(Vec<u16>),
    Ucs2(Vec<u16>),
}
