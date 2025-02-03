use std::collections::HashMap;

use crate::assembler::{AssemblerError, Backtrace};

use super::Statement;

pub struct LoadImmediate {
    destination_register: u16,
    value: u16,
}

impl LoadImmediate {
    pub fn new(destination_register: u16, value: u16) -> LoadImmediate {
        LoadImmediate {
            destination_register,
            value,
        }
    }
}

impl Statement for LoadImmediate {
    fn assemble(&self, _: u16, _: &HashMap<String, u16>, _: &Backtrace) -> Result<Vec<u16>, AssemblerError> {
        return Ok(vec![(0b0111 << 12) | (self.destination_register << 9) | self.value]);
    }

    fn width(&self) -> u16 {
        return 1;
    }
}
