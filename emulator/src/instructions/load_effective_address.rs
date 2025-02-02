use std::fmt::{Debug, Formatter, Result as FormatResult};

use crate::{decode_signed_integer, state::State};

use super::{Instruction, Register};

pub struct LoadEffectiveAddress {
    pub destination_register: Register,
    pub offset: i16,
}

impl Instruction for LoadEffectiveAddress {
    fn new(machine_code: u16) -> LoadEffectiveAddress {
        let destination_register = ((machine_code >> 9) & 0b111) as Register;
        let encoded_offset = machine_code & 0b111111111;

        let offset = decode_signed_integer!(encoded_offset, 9);

        LoadEffectiveAddress {
            destination_register,
            offset,
        }
    }

    fn execute(&self, state: &mut State) {
        let effective_address = match self.offset < 0 {
            true => state.pc.wrapping_add(self.offset as u16),
            false => state.pc.wrapping_sub((0 - self.offset) as u16),
        };

        state.set_register_and_flags(self.destination_register, effective_address);
    }
}

impl Debug for LoadEffectiveAddress {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "LEA R{} #{}", self.destination_register, self.offset)
    }
}
