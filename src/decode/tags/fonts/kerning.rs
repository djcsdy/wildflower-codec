#[derive(Clone, PartialEq, Debug)]
pub struct KerningRecord<T> {
    pub left_character_code: T,
    pub right_character_code: T,
    pub kerning_adjustment: i16,
}
