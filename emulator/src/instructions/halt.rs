use std::fmt::{Debug, Formatter, Result as FormatResult};

use crate::state::State;

use super::Instruction;

pub struct Halt {}

impl Instruction for Halt {
    fn new(_machine_code: u16) -> Halt {
        Halt {}
    }

    fn execute(&self, state: &mut State) {
        state.halt = true;
    }
}

impl Debug for Halt {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "HALT")
    }
}
