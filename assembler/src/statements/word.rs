use std::{collections::HashMap, ops::Range};

use crate::assembler::AssemblerError;

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
    fn assemble(&self, _: u16, _: &HashMap<String, u16>, _: &Range<usize>) -> Result<Vec<u16>, AssemblerError> {
        return Ok(vec![self.value]);
    }

    fn width(&self) -> u16 {
        return 1;
    }
}
