mod add;
mod sub;
mod load_effective_address;
mod load;
mod load_immediate;
mod store;
mod branch;
mod call;
mod r#return;
mod halt;
mod sleep;
mod word;
mod ascii;

pub use add::Add;
pub use sub::Sub;
pub use load_effective_address::LoadEffectiveAddress;
pub use load::Load;
pub use load_immediate::LoadImmediate;
pub use store::Store;
pub use branch::Branch;
pub use call::Call;
pub use r#return::Return;
pub use halt::Halt;
pub use sleep::Sleep;
pub use word::Word;
pub use ascii::Ascii;

use std::{collections::HashMap, ops::Range};

use crate::assembler::AssemblerError;

pub trait Statement {
    fn assemble(&self, address: u16, label_map: &HashMap<String, u16>, span: &Range<usize>) -> Result<Vec<u16>, AssemblerError>;
    fn width(&self) -> u16;
}

#[derive(Clone)]
pub struct StatementContainer<T: ?Sized + Statement> {
    statement: Box<T>,
    span: Range<usize>
}

impl StatementContainer<dyn Statement> {
    pub fn new(statement: Box<dyn Statement>, span: Range<usize>) -> Self {
        StatementContainer {
            statement,
            span
        }
    }

    pub fn assemble(&self, address: u16, label_map: &HashMap<String, u16>) -> Result<Vec<u16>, AssemblerError> {
        self.statement.assemble(address, label_map, &self.span)
    }

    pub fn width(&self) -> u16 {
        self.statement.width()
    }
}