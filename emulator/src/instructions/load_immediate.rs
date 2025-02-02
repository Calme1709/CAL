use std::fmt::{Debug, Formatter, Result as FormatResult};

use crate::state::State;

use super::{Instruction, Register};

pub struct LoadImmediate {
    pub dr: Register,
    pub immediate: u16,
}

impl Instruction for LoadImmediate {
    fn new(machine_code: u16) -> LoadImmediate {
        let dr = ((machine_code >> 9) & 0b111) as Register;
        let immediate = machine_code & 0b111111111;

        LoadImmediate { dr, immediate }
    }

    fn execute(&self, state: &mut State) {
        state.set_register_and_flags(self.dr, self.immediate);
    }
}

impl Debug for LoadImmediate {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "LDI R{} #{}", self.dr, self.immediate)?;

        Ok(())
    }
}
