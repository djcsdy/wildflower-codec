use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tags::buttons::button_condition_key_press::ButtonConditionKeyPress;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct ButtonCondition {
    flags: u16,
}

impl ButtonCondition {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let flags = reader.read_u16()?;
        Ok(Self { flags })
    }

    pub fn idle_to_over_down(&self) -> bool {
        self.flags & 0x8000 == 0x8000
    }

    pub fn out_down_to_idle(&self) -> bool {
        self.flags & 0x4000 == 0x4000
    }

    pub fn out_down_to_over_down(&self) -> bool {
        self.flags & 0x2000 == 0x2000
    }

    pub fn over_down_to_out_down(&self) -> bool {
        self.flags & 0x1000 == 0x1000
    }

    pub fn over_down_to_over_up(&self) -> bool {
        self.flags & 0x0800 == 0x0800
    }

    pub fn over_up_to_over_down(&self) -> bool {
        self.flags & 0x0400 == 0x0400
    }

    pub fn over_up_to_idle(&self) -> bool {
        self.flags & 0x0200 == 0x0200
    }

    pub fn idle_to_over_up(&self) -> bool {
        self.flags & 0x0100 == 0x0100
    }

    pub fn key_press(&self) -> ButtonConditionKeyPress {
        ButtonConditionKeyPress::from(((self.flags >> 1) & 0x7f) as u8)
    }

    pub fn over_down_to_idle(&self) -> bool {
        self.flags & 0x0001 == 0x0001
    }
}
