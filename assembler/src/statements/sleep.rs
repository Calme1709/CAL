use std::collections::HashMap;

use crate::assembler::{AssemblerError, Backtrace};

use super::Statement;

pub struct Sleep {
    duration: u16,
}

impl Sleep {
    pub fn new(duration: u16) -> Sleep {
        Sleep { duration }
    }
}

impl Statement for Sleep {
    fn assemble(&self, _: u16, _: &HashMap<String, u16>, _: &Backtrace) -> Result<Vec<u16>, AssemblerError> {
        return Ok(vec![(0b1101 << 12) | self.duration]);
    }

    fn width(&self) -> u16 {
        return 1;
    }
}
