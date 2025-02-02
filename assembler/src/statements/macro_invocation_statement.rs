use std::{collections::HashMap, ops::Range};

use crate::assembler::AssemblerError;

use super::{Statement, StatementContainer};

pub struct MacroInvocationStatement {
    contained_statements: Vec<StatementContainer<dyn Statement>>,
}

impl MacroInvocationStatement {
    pub fn new(contained_statements: Vec<StatementContainer<dyn Statement>>) -> MacroInvocationStatement {
        MacroInvocationStatement { contained_statements }
    }
}

impl Statement for MacroInvocationStatement {
    fn assemble(
        &self,
        address: u16,
        label_map: &HashMap<String, u16>,
        _: &Range<usize>,
    ) -> Result<Vec<u16>, AssemblerError> {
        let assembled_statements = self
            .contained_statements
            .iter()
            .map(|statement| statement.assemble(address, label_map));

        let mut output: Vec<u16> = Vec::new();

        for assembled_statement in assembled_statements {
            output.append(&mut assembled_statement?);
        }

        Ok(output)
    }

    fn width(&self) -> u16 {
        let mut width: u16 = 0;

        for statement in &self.contained_statements {
            width += statement.width();
        }

        return width;
    }
}
