use std::fmt::{Debug, Formatter, Result as FormatResult};

use crate::{decode_signed_integer, state::State};

use super::{Instruction, Register};

pub struct Store {
    base_register: Register,
    offset: i16,
    source_register: Register,
}

impl Instruction for Store {
    fn new(machine_code: u16) -> Store {
        Store {
            base_register: ((machine_code >> 9) & 0b111) as Register,
            offset: decode_signed_integer!((machine_code >> 3) & 0b111111, 6),
            source_register: (machine_code & 0b111) as Register
        }
    }

    fn execute(&self, state: &mut State) {
        let base_register_value = state.registers[self.base_register as usize];

        let address = match self.offset >= 0 {
            true => base_register_value.wrapping_add(self.offset as u16),
            false => base_register_value.wrapping_sub((0 - self.offset) as u16)
        };

        state.memory[address as usize] = state.registers[self.source_register as usize];
    }
}

impl Debug for Store {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "ST R{} #{} R{}", self.base_register, self.offset, self.source_register)
    }
}