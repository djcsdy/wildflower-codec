#[derive(Clone, PartialEq, Debug)]
pub struct KerningRecord<CharacterCode> {
    pub left_character_code: CharacterCode,
    pub right_character_code: CharacterCode,
    pub kerning_adjustment: i16,
}
