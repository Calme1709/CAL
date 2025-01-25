mod add;
mod halt;
mod load;
mod sub;
mod branch;
mod store;
mod load_immediate;

use std::fmt::Debug;

use add::Add;
use halt::Halt;
use sub::Sub;
use load::Load;
use branch::Branch;
use store::Store;
use load_immediate::LoadImmediate;

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
        0x1 => Box::new(Sub::new(machine_code)),
        0x6 => Box::new(Load::new(machine_code)),
        0x7 => Box::new(LoadImmediate::new(machine_code)),
        0x8 => Box::new(Store::new(machine_code)),
        0x9 => Box::new(Branch::new(machine_code)),
        0xC => Box::new(Halt::new(machine_code)),
        _ => panic!("Invalid opcode {:1X}", (machine_code >> 12) & 0xF), 
    }
}