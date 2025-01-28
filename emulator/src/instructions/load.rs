use std::fmt::{Debug, Formatter, Result as FormatResult};

use crate::{decode_signed_integer, state::State};

use super::{Instruction, Register};

pub struct Load {
    destination_register: Register,
    base_register: Register,
    offset: i16
}

impl Instruction for Load {
    fn new(machine_code: u16) -> Load {
        Load {
            destination_register: ((machine_code >> 9) & 0b111) as Register,
            base_register: ((machine_code >> 6) & 0b111) as Register,
            offset: decode_signed_integer!(machine_code & 0b111111, 6)
        }
    }

    fn execute(&self, state: &mut State) {
        let base_register_value = state.registers[self.base_register as usize];

        let address = match self.offset >= 0 {
            true => base_register_value.wrapping_add(self.offset as u16),
            false => base_register_value.wrapping_sub((0 - self.offset) as u16)
        };

        let value = match address {
            0xFFFE => {
                match state.stdin.len() {
                    0 => 0x80 as u16,
                    _ => state.stdin.remove(0) as u16
                }
            },
            _ => state.memory[address as usize]
        };

        state.set_register_and_flags(self.destination_register, value);
    }
}

impl Debug for Load {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "LD R{} R{} #{}", self.destination_register, self.base_register, self.offset)
    }
}