mod add;
mod ascii;
mod block;
mod branch;
mod call;
mod halt;
mod load;
mod load_effective_address;
mod load_immediate;
mod macro_invocation_statement;
mod r#return;
mod sleep;
mod store;
mod sub;
mod word;

pub use add::Add;
pub use ascii::Ascii;
pub use block::Block;
pub use branch::Branch;
pub use call::Call;
pub use halt::Halt;
pub use load::Load;
pub use load_effective_address::LoadEffectiveAddress;
pub use load_immediate::LoadImmediate;
pub use macro_invocation_statement::MacroInvocationStatement;
pub use r#return::Return;
pub use sleep::Sleep;
pub use store::Store;
pub use sub::Sub;
pub use word::Word;

use std::{collections::HashMap, ops::Range};

use crate::assembler::AssemblerError;

pub trait Statement {
    fn assemble(
        &self,
        address: u16,
        label_map: &HashMap<String, u16>,
        span: &Range<usize>,
    ) -> Result<Vec<u16>, AssemblerError>;
    fn width(&self) -> u16;
}

#[derive(Clone)]
pub struct StatementContainer<T: ?Sized + Statement> {
    statement: Box<T>,
    span: Range<usize>,
}

impl StatementContainer<dyn Statement> {
    pub fn new(statement: Box<dyn Statement>, span: Range<usize>) -> Self {
        StatementContainer { statement, span }
    }

    pub fn assemble(&self, address: u16, label_map: &HashMap<String, u16>) -> Result<Vec<u16>, AssemblerError> {
        self.statement.assemble(address, label_map, &self.span)
    }

    pub fn width(&self) -> u16 {
        self.statement.width()
    }
}
