use std::fmt::{Debug, Formatter, Result as FormatResult};

use crate::{decode_signed_integer, state::State};

use super::{Instruction, Register};

struct PCRelativeCallParams {
    offset: i16,
}

struct RegisterRelativeCallParams {
    base_register: Register,
    offset: i16,
}

enum CallParams {
    PCRelative(PCRelativeCallParams),
    RegisterRelative(RegisterRelativeCallParams),
}

pub struct Call(CallParams);

impl Instruction for Call {
    fn new(machine_code: u16) -> Call {
        let params = match (machine_code >> 11) & 0b1 {
            0 => CallParams::RegisterRelative(RegisterRelativeCallParams {
                base_register: ((machine_code >> 8) & 0b111) as u8,
                offset: decode_signed_integer!(machine_code & 0b11111111, 8),
            }),
            1 => CallParams::PCRelative(PCRelativeCallParams {
                offset: decode_signed_integer!(machine_code & 0b11111111111, 11),
            }),
            _ => unreachable!(),
        };

        Call(params)
    }

    fn execute(&self, state: &mut State) {
        let (base_address, offset) = match &self.0 {
            CallParams::RegisterRelative(params) => (state.registers[params.base_register as usize], params.offset),
            CallParams::PCRelative(params) => (state.pc, params.offset),
        };

        let destination_address = base_address.wrapping_add_signed(offset);

        state.call_stack[state.call_stack_pointer as usize] = state.pc;
        state.call_stack_pointer += 1;

        state.pc = destination_address;
    }
}

impl Debug for Call {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "CALL ")?;

        match &self.0 {
            CallParams::PCRelative(params) => write!(f, "#{}", params.offset)?,
            CallParams::RegisterRelative(params) => write!(f, "R{} #{}", params.base_register, params.offset)?,
        }

        Ok(())
    }
}
