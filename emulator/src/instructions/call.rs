use std::fmt::{Debug, Formatter, Result as FormatResult};

use crate::state::State;

use super::Instruction;

pub struct Call {
    subroutine_lookup_table_index: u16,
}

impl Instruction for Call {
    fn new(machine_code: u16) -> Call {
        Call {
            subroutine_lookup_table_index: machine_code & 0b111111111111,
        }
    }

    fn execute(&self, state: &mut State) {
        let subroutine_address = state.memory[1 + self.subroutine_lookup_table_index as usize];

        state.call_stack[state.call_stack_pointer as usize] = state.pc;
        state.call_stack_pointer += 1;

        state.pc = subroutine_address;
    }
}

impl Debug for Call {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "CALL {}", self.subroutine_lookup_table_index)
    }
}
