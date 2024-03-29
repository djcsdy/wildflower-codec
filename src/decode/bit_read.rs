use std::io::{Read, Result};

pub trait BitRead: Read {
    fn align_byte(&mut self);

    fn read_bit(&mut self) -> Result<bool> {
        Ok(self.read_ub(1)? == 1)
    }

    fn read_ub8(&mut self, bits: u8) -> Result<u8> {
        if bits > 8 {
            panic!();
        }

        Ok(self.read_ub(bits)? as u8)
    }

    fn read_ub16(&mut self, bits: u8) -> Result<u16> {
        if bits > 16 {
            panic!();
        }

        Ok(self.read_ub(bits)? as u16)
    }

    fn read_ub(&mut self, bits: u8) -> Result<u32>;

    fn read_sb16(&mut self, bits: u8) -> Result<i16> {
        if bits > 16 {
            panic!();
        }

        Ok(self.read_sb(bits)? as i16)
    }

    fn read_sb(&mut self, bits: u8) -> Result<i32> {
        Ok(extend_sign(self.read_ub(bits)?, bits))
    }
}

pub fn extend_sign(value: u32, bits: u8) -> i32 {
    let sign = 1u32 << (bits - 1);
    if (value & sign) == 0 {
        value as i32
    } else {
        (value as i32) | -(sign as i32)
    }
}

pub struct BitReadOptions<'read_byte, ReadByte: FnMut() -> Result<u8>> {
    pub read_byte: &'read_byte mut ReadByte,
    pub state: BitReadState,
    pub bits: u8,
}

pub struct BitReadState {
    pub partial_byte: u8,
    pub partial_bit_count: u8,
}

pub fn bit_read<ReadByte: FnMut() -> Result<u8>>(
    options: BitReadOptions<ReadByte>,
) -> (BitReadState, Result<u32>) {
    bit_read_internal(options)
        .map(|(state, value)| (state, Ok(value)))
        .unwrap_or_else(|err| {
            (
                BitReadState {
                    partial_byte: 0,
                    partial_bit_count: 0,
                },
                Err(err),
            )
        })
}

fn bit_read_internal<ReadByte: FnMut() -> Result<u8>>(
    BitReadOptions {
        read_byte,
        state:
            BitReadState {
                mut partial_byte,
                mut partial_bit_count,
            },
        bits,
    }: BitReadOptions<ReadByte>,
) -> Result<(BitReadState, u32)> {
    if bits > 32 {
        panic!();
    }

    if bits <= partial_bit_count {
        Ok((
            BitReadState {
                partial_byte,
                partial_bit_count: partial_bit_count - bits,
            },
            (partial_byte as u32) >> partial_bit_count,
        ))
    } else {
        let mut result = partial_byte as u32;
        let mut bits_remaining = bits - partial_bit_count;
        while bits_remaining > 8 {
            result = (result << 8) | read_byte()? as u32;
            bits_remaining = bits_remaining - 8;
        }

        partial_byte = read_byte()?;
        partial_bit_count = 8 - bits_remaining;

        Ok((
            BitReadState {
                partial_byte,
                partial_bit_count,
            },
            (result << bits_remaining) | ((partial_byte as u32) >> partial_bit_count),
        ))
    }
}
