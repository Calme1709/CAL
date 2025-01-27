use std::{fmt::{Debug, Formatter, Result as FormatResult}, thread, time::Duration};

use crate::state::State;

use super::Instruction;

pub struct Sleep {
    duration: u16
}

impl Instruction for Sleep {
    fn new(machine_code: u16) -> Sleep {
        Sleep {
            duration: machine_code & 0b111111111111
        }
    }

    fn execute(&self, _: &mut State) {
        thread::sleep(Duration::from_millis(self.duration.into()));
    }
}

impl Debug for Sleep {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "SLP #{}", self.duration)
    }
}