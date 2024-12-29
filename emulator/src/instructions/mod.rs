mod add;
mod halt;

use std::fmt::Debug;

use add::Add;
use halt::Halt;

use crate::state::State;

pub type Register = u8;
pub type Imm5 = u8;

#[derive(PartialEq, Eq, Debug)]
pub enum ArtihmeticMode {
    Register,
    Immediate,
}

pub trait Instruction : Debug {
    fn new(machine_code: u16) -> Self where Self : Sized;
    fn execute(&self, state: &mut State);
}

pub fn from_machine_code(machine_code: u16) -> Box<dyn Instruction> {
    match (machine_code >> 12) & 0xF {
        0x0 => Box::new(Add::new(machine_code)),
        0xC => Box::new(Halt::new(machine_code)),
        _ => panic!("Invalid opcode {:1X}", (machine_code >> 12) & 0xF), 
    }
}