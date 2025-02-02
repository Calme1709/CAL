use std::fmt::{Debug, Formatter, Result as FormatResult};

use crate::state::State;

use super::{ArtihmeticMode, Imm5, Instruction, Register};

pub struct Sub {
    pub dr: Register,
    pub sr0: Register,
    pub mode: ArtihmeticMode,
    pub sr1: Register,
    pub imm5: Imm5,
}

impl Instruction for Sub {
    fn new(machine_code: u16) -> Sub {
        let dr = ((machine_code >> 9) & 0b111) as Register;
        let sr0 = ((machine_code >> 6) & 0b111) as Register;
        let mode = match (machine_code >> 5) & 0b1 {
            0 => ArtihmeticMode::Immediate,
            1 => ArtihmeticMode::Register,
            _ => panic!("Invalid mode"),
        };
        let sr1 = ((machine_code >> 2) & 0b111) as Register;
        let imm5 = (machine_code & 0b11111) as Imm5;

        Sub {
            dr,
            sr0,
            mode,
            sr1,
            imm5,
        }
    }

    fn execute(&self, state: &mut State) {
        let first_value = state.registers[self.sr0 as usize];
        let second_value = match self.mode {
            ArtihmeticMode::Register => state.registers[self.sr1 as usize],
            ArtihmeticMode::Immediate => self.imm5 as u16,
        };

        state.set_register_and_flags(self.dr, first_value.wrapping_sub(second_value));
    }
}

impl Debug for Sub {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "SUB R{}, ", self.dr)?;
        write!(f, "R{}, ", self.sr0)?;

        match self.mode {
            ArtihmeticMode::Register => write!(f, "R{}", self.sr1)?,
            ArtihmeticMode::Immediate => write!(f, "#{}", self.imm5)?,
        }

        Ok(())
    }
}
