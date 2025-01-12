use std::fmt::{Debug, Formatter, Result as FormatResult};

use shared::BranchConditions;

use crate::state::State;

use super::Instruction;

// FIXME: There is probably a non-iterative way to do this
macro_rules! decode_signed_integer {
    ( $encoded:expr, $bits:expr ) => {{
        (if $encoded & 1 << ($bits - 1) != 0 { (((2_u16).pow(16 - $bits) - 1) << $bits) } else { 0 } | $encoded) as i16
    }}
}

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