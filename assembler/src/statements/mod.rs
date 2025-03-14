mod add;
mod ascii;
mod block;
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
pub use r#return::Return;
pub use sleep::Sleep;
pub use store::Store;
pub use sub::Sub;
pub use word::Word;

use std::collections::HashMap;

use crate::assembler::{AssemblerError, Backtrace};

pub trait Statement {
    fn assemble(
        &self,
        address: u16,
        label_map: &HashMap<String, u16>,
        subroutine_lookup_table_entries: &Vec<String>,
        span: &Backtrace,
    ) -> Result<Vec<u16>, AssemblerError>;
    fn width(&self) -> u16;
}

#[derive(Clone)]
pub struct StatementContainer<T: ?Sized + Statement> {
    statement: Box<T>,
    backtrace: Backtrace,
}

impl StatementContainer<dyn Statement> {
    pub fn new(statement: Box<dyn Statement>, backtrace: Backtrace) -> Self {
        StatementContainer { statement, backtrace }
    }

    pub fn assemble(
        &self,
        address: u16,
        label_map: &HashMap<String, u16>,
        subroutine_lookup_table_entries: &Vec<String>,
    ) -> Result<Vec<u16>, AssemblerError> {
        self.statement
            .assemble(address, label_map, subroutine_lookup_table_entries, &self.backtrace)
    }

    pub fn width(&self) -> u16 {
        self.statement.width()
    }
}
