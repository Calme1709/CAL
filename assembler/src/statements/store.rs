use std::{collections::HashMap, ops::Range};

use crate::{assembler::AssemblerError, encode_signed_integer};

use super::Statement;

pub struct Store {
    base_register: u16,
    offset: i32,
    source_register: u16
}

impl Store {
    pub fn new(base_register: u16, offset: i32, source_register: u16) -> Store {
        Store {
            base_register,
            offset,
            source_register
        }
    }
}

impl Statement for Store {
    fn assemble(&self, _: u16, _: &HashMap<String, u16>, span: &Range<usize>) -> Result<Vec<u16>, AssemblerError> {
        return Ok(vec![(0b1000 << 12) | (self.base_register << 9) | (encode_signed_integer!(self.offset, 6, span.clone())? << 3) | self.source_register]);
    }

    fn width(&self) -> u16 {
        return 1;
    }
}