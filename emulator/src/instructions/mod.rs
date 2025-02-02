mod add;
mod branch;
mod call;
mod halt;
mod load;
mod load_effective_address;
mod load_immediate;
mod r#return;
mod sleep;
mod store;
mod sub;

use std::fmt::Debug;

use add::Add;
use branch::Branch;
use call::Call;
use halt::Halt;
use load::Load;
use load_effective_address::LoadEffectiveAddress;
use load_immediate::LoadImmediate;
use r#return::Return;
use sleep::Sleep;
use store::Store;
use sub::Sub;

use crate::state::State;

pub type Register = u8;
pub type Imm5 = u8;

#[derive(PartialEq, Eq, Debug)]
pub enum ArtihmeticMode {
    Register,
    Immediate,
}

pub trait Instruction: Debug {
    fn new(machine_code: u16) -> Self
    where
        Self: Sized;
    fn execute(&self, state: &mut State);
}

pub fn from_machine_code(machine_code: u16) -> Box<dyn Instruction> {
    match (machine_code >> 12) & 0xF {
        0x0 => Box::new(Add::new(machine_code)),
        0x1 => Box::new(Sub::new(machine_code)),
        0x5 => Box::new(LoadEffectiveAddress::new(machine_code)),
        0x6 => Box::new(Load::new(machine_code)),
        0x7 => Box::new(LoadImmediate::new(machine_code)),
        0x8 => Box::new(Store::new(machine_code)),
        0x9 => Box::new(Branch::new(machine_code)),
        0xA => Box::new(Call::new(machine_code)),
        0xB => Box::new(Return::new(machine_code)),
        0xC => Box::new(Halt::new(machine_code)),
        0xD => Box::new(Sleep::new(machine_code)),
        _ => panic!("Invalid opcode {:1X}", (machine_code >> 12) & 0xF),
    }
}
