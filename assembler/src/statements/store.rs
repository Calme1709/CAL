use std::collections::HashMap;

use crate::{
    assembler::{AssemblerError, Backtrace},
    utils::encode_signed_integer,
};

use super::Statement;

pub struct Store {
    base_register: u16,
    offset: i32,
    source_register: u16,
}

impl Store {
    pub fn new(base_register: u16, offset: i32, source_register: u16) -> Store {
        Store {
            base_register,
            offset,
            source_register,
        }
    }
}

impl Statement for Store {
    fn assemble(
        &self,
        _: u16,
        _: &HashMap<String, u16>,
        _: &Vec<String>,
        backtrace: &Backtrace,
    ) -> Result<Vec<u16>, AssemblerError> {
        let encoded_offset = match encode_signed_integer(self.offset, 6) {
            Ok(value) => value,
            Err(e) => return Err(AssemblerError::new(e, backtrace.clone())),
        };

        return Ok(vec![
            (0b1000 << 12) | (self.base_register << 9) | (encoded_offset << 3) | self.source_register,
        ]);
    }

    fn width(&self) -> u16 {
        return 1;
    }
}
