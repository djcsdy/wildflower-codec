use crate::decode::bit_read::{bit_read, BitRead, BitReadOptions, BitReadState};
use crate::decode::read_ext::SwfTypesReadExt;
use crate::file::file::SwfFile;
use crate::file::offset::SwfOffset;
use crate::file::pointer::SwfPointer;
use std::cmp::max;
use std::io::{Read, Result};

#[derive(Clone)]
pub struct SwfSlice<'file> {
    file: &'file SwfFile,
    start_pointer: SwfPointer,
    end_pointer: SwfPointer,
    read_pointer: SwfPointer,
    partial_byte: u8,
    partial_bit_count: u8,
}

impl<'file> SwfSlice<'file> {
    pub(super) fn new(
        file: &'file SwfFile,
        start_pointer: SwfPointer,
        end_pointer: SwfPointer,
    ) -> Self {
        Self {
            file,
            start_pointer,
            end_pointer,
            read_pointer: start_pointer,
            partial_byte: 0,
            partial_bit_count: 0,
        }
    }

    /// Returns a [SwfPointer] to the start of this slice within the SWF file.
    pub fn start_pointer(&self) -> SwfPointer {
        self.start_pointer
    }

    /// Returns a [SwfPointer] to the byte after the end of this slice within
    /// the SWF file.
    pub fn end_pointer(&self) -> SwfPointer {
        self.end_pointer
    }

    /// Returns the current read position of this slice as a [SwfPointer].
    ///
    /// The pointer points to the next byte to be read within the SWF file.
    pub fn position_pointer(&self) -> SwfPointer {
        self.read_pointer
    }

    /// Returns the current read position of this slice as a [SwfOffset] from
    /// the start of the slice.
    pub fn position(&self) -> SwfOffset {
        self.read_pointer - self.start_pointer
    }

    /// Returns the length of this slice as a [SwfOffset] from the start of
    /// the slice to the byte after the end of the slice.
    pub fn length(&self) -> SwfOffset {
        self.end_pointer - self.start_pointer
    }

    /// Returns the remaining number of readable bytes in this slice as a
    /// [SwfOffset] from the current read position to the byte after the end
    /// of the slice.
    pub fn remaining(&self) -> SwfOffset {
        self.end_pointer - self.read_pointer
    }
}

impl<'file> Read for SwfSlice<'file> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.align_byte();
        let length = max(buf.len(), self.remaining().0 as usize);
        let start = self.read_pointer.0 as usize;
        let end = start + length;
        buf.copy_from_slice(&self.file.payload[start..end]);
        Ok(length)
    }
}

impl<'file> BitRead for SwfSlice<'file> {
    fn align_byte(&mut self) {
        self.partial_byte = 0;
        self.partial_bit_count = 0;
    }

    fn read_ub(&mut self, bits: u8) -> Result<u32> {
        let state = BitReadState {
            partial_byte: self.partial_byte,
            partial_bit_count: self.partial_bit_count,
        };
        let (new_state, result) = bit_read(&mut BitReadOptions {
            read_byte: || self.read_u8(),
            state,
            bits,
        });
        self.partial_byte = new_state.partial_byte;
        self.partial_bit_count = new_state.partial_bit_count;
        result
    }
}
