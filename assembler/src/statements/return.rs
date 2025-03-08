use std::collections::HashMap;

use crate::assembler::{AssemblerError, Backtrace};

use super::Statement;

pub struct Return {}

impl Return {
    pub fn new() -> Return {
        Return {}
    }
}

impl Statement for Return {
    fn assemble(
        &self,
        _: u16,
        _: &HashMap<String, u16>,
        _: &Vec<String>,
        _: &Backtrace,
    ) -> Result<Vec<u16>, AssemblerError> {
        return Ok(vec![0b1011000000000000]);
    }

    fn width(&self) -> u16 {
        return 1;
    }
}
