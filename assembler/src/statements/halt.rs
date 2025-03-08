use std::collections::HashMap;

use crate::assembler::{AssemblerError, Backtrace};

use super::Statement;

pub struct Halt {}

impl Halt {
    pub fn new() -> Halt {
        Halt {}
    }
}

impl Statement for Halt {
    fn assemble(
        &self,
        _: u16,
        _: &HashMap<String, u16>,
        _: &Vec<String>,
        _: &Backtrace,
    ) -> Result<Vec<u16>, AssemblerError> {
        return Ok(vec![0b1100000000000000]);
    }

    fn width(&self) -> u16 {
        return 1;
    }
}
