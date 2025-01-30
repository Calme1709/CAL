use std::fmt::{Debug, Formatter, Result as FormatResult};

use crate::state::State;

use super::Instruction;

pub struct Return {}

impl Instruction for Return {
    fn new(_machine_code: u16) -> Return {
        Return {}
    }

    fn execute(&self, state: &mut State) {
        state.call_stack_pointer -= 1;
        state.pc = state.call_stack[state.call_stack_pointer as usize];
    }
}

impl Debug for Return {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "RET")
    }
}