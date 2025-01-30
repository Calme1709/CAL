use std::{collections::HashMap, ops::Range};

use crate::assembler::AssemblerError;

use super::Statement;

pub struct Block {
    size: u16,
}

impl Block {
    pub fn new(size: u16) -> Block {
        Block {
            size,
        }
    }
}

impl Statement for Block {
    fn assemble(&self, _: u16, _: &HashMap<String, u16>, _: &Range<usize>) -> Result<Vec<u16>, AssemblerError> {
        return Ok(vec![0; self.size as usize]);
    }

    fn width(&self) -> u16 {
        return self.size;
    }
}