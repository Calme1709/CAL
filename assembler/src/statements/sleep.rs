use std::{collections::HashMap, ops::Range};

use crate::assembler::AssemblerError;

use super::Statement;

pub struct Sleep {
    duration: u16
}

impl Sleep {
    pub fn new(duration: u16) -> Sleep {
        Sleep {
            duration
        }
    }
}

impl Statement for Sleep {
    fn assemble(&self, _: u16, _: &HashMap<String, u16>, _: &Range<usize>) -> Result<Vec<u16>, AssemblerError> {
        return Ok(vec![(0b1101 << 12) | self.duration]);
    }

    fn width(&self) -> u16 {
        return 1;
    }
}