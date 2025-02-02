use std::{collections::HashMap, ops::Range};

use crate::{assembler::AssemblerError, encode_signed_integer};

use super::Statement;

pub struct Load {
    destination_register: u16,
    base_register: u16,
    offset: i32,
}

impl Load {
    pub fn new(destination_register: u16, base_register: u16, offset: i32) -> Load {
        Load {
            destination_register,
            base_register,
            offset,
        }
    }
}

impl Statement for Load {
    fn assemble(&self, _: u16, _: &HashMap<String, u16>, span: &Range<usize>) -> Result<Vec<u16>, AssemblerError> {
        return Ok(vec![
            (0b0110 << 12)
                | (self.destination_register << 9)
                | (self.base_register << 6)
                | encode_signed_integer!(self.offset, 6, span.clone())?,
        ]);
    }

    fn width(&self) -> u16 {
        return 1;
    }
}
