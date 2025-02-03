use std::collections::HashMap;

use crate::assembler::{AssemblerError, Backtrace};

use super::Statement;

pub struct Word {
    value: u16,
}

impl Word {
    pub fn new(value: u16) -> Word {
        Word { value }
    }
}

impl Statement for Word {
    fn assemble(&self, _: u16, _: &HashMap<String, u16>, _: &Backtrace) -> Result<Vec<u16>, AssemblerError> {
        return Ok(vec![self.value]);
    }

    fn width(&self) -> u16 {
        return 1;
    }
}
