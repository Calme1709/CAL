use std::collections::HashMap;

use crate::assembler::{AssemblerError, Backtrace};

use super::Statement;

pub struct Block {
    size: u16,
}

impl Block {
    pub fn new(size: u16) -> Block {
        Block { size }
    }
}

impl Statement for Block {
    fn assemble(
        &self,
        _: u16,
        _: &HashMap<String, u16>,
        _: &Vec<String>,
        _: &Backtrace,
    ) -> Result<Vec<u16>, AssemblerError> {
        return Ok(vec![0; self.size as usize]);
    }

    fn width(&self) -> u16 {
        return self.size;
    }
}
