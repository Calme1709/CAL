use std::fmt::{Debug, Formatter, Result as FormatResult};

use shared::BranchConditions;

use crate::{decode_signed_integer, state::State};

use super::Instruction;

pub struct Branch {
    pub conditions: BranchConditions,
    pub offset: i16,
}

impl Instruction for Branch {
    fn new(machine_code: u16) -> Branch {
        let conditions = BranchConditions::from_bits((machine_code >> 9) & 0b111).unwrap();
        let encoded_offset = machine_code & 0b111111111;

        let offset = decode_signed_integer!(encoded_offset, 9);

        Branch {
            conditions,
            offset
        }
    }

    fn execute(&self, state: &mut State) {
        if state.flags & self.conditions != BranchConditions::empty() {
            if self.offset < 0 {
                state.pc = state.pc.wrapping_add(self.offset as u16);
            } else {
                state.pc = state.pc.wrapping_sub((0 - self.offset) as u16);
            }
        }
    }
}

impl Debug for Branch {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "BR {} {}", self.conditions.as_string(), self.offset)?;

        Ok(())
    }
}