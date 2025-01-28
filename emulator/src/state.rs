use std::fmt::{
    Formatter,
    Result as FormatResult,
    Debug
};

use shared::BranchConditions;

pub struct State {
    pub memory: [u16; 65536],
    pub call_stack: [u8; 256],
    pub call_stack_pointer: u8,
    pub registers: [u16; 8],
    pub pc: u16,
    pub halt: bool,
    pub flags: BranchConditions,
    pub stdin: Vec<u8>,
}

impl State {
    pub fn new() -> State {
        State {
            memory: [0; 65536],
            call_stack: [0; 256],
            call_stack_pointer: 0,
            registers: [0; 8],
            pc: 0,
            halt: false,
            flags: BranchConditions::empty(),
            stdin: Vec::new()
        }
    }

    pub fn set_register_and_flags(&mut self, register: u8, value: u16) {
        self.registers[register as usize] = value;

        if value == 0 {
            self.flags = BranchConditions::ZERO;
        } else if value & 0x8000 == 0x8000 {
            self.flags = BranchConditions::NEGATIVE;
        } else {
            self.flags = BranchConditions::POSITIVE;
        }
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        for i in 0..8 {
            writeln!(f, "R{}: {}", i, self.registers[i])?;
        }

        writeln!(f, "PC: {}", self.pc)?;

        writeln!(f)?;

        writeln!(f, "HALT: {}", self.halt)?;

        writeln!(f)?;

        writeln!(f, "Call Stack:")?;

        for i in (0..self.call_stack_pointer).rev() {
            writeln!(f, "{:04X}: {:02X}", i, self.call_stack[i as usize])?;
        }

        writeln!(f)?;

        writeln!(f, "Flags: {:03b}", self.flags)?;

        writeln!(f)?;

        writeln!(f, "Memory:")?;

        let mut last_line = "".to_string();
        let mut already_output_truncated = false;

        for i in 0..4096 {
            let line_output = self.memory[i * 16..i * 16 + 16].iter().map(|x| format!("{:04X}", x)).collect::<Vec<String>>().join(" ");

            if line_output == last_line && i != 0 && i != 4095 {
                if already_output_truncated {
                    continue;
                }

                writeln!(f, "*")?;
                already_output_truncated = true;
                continue;
            }

            writeln!(f, "{:04X}: {}", i * 16, line_output)?;

            last_line = line_output;
            already_output_truncated = false;
        }

        Ok(())
    }
}