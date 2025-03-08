use std::collections::HashMap;

use crate::assembler::{AssemblerError, Backtrace};

use super::Statement;

pub struct Add {
    destination_register: u16,
    source_register_zero: u16,
    source_one_value: u16,
}

impl Add {
    pub fn new(destination_register: u16, source_register_zero: u16, source_one_value: u16) -> Add {
        Add {
            destination_register,
            source_register_zero,
            source_one_value,
        }
    }
}

impl Statement for Add {
    fn assemble(
        &self,
        _: u16,
        _: &HashMap<String, u16>,
        _: &Vec<String>,
        _: &Backtrace,
    ) -> Result<Vec<u16>, AssemblerError> {
        return Ok(vec![
            (0b0000 << 12)
                | (self.destination_register << 9)
                | (self.source_register_zero << 6)
                | self.source_one_value,
        ]);
    }

    fn width(&self) -> u16 {
        return 1;
    }
}
