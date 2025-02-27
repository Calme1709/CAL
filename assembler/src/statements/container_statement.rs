use std::collections::HashMap;

use crate::assembler::{AssemblerError, Backtrace};

use super::{Statement, StatementContainer};

pub struct ContainerStatement {
    contained_statements: Vec<StatementContainer<dyn Statement>>,
}

impl ContainerStatement {
    pub fn new(contained_statements: Vec<StatementContainer<dyn Statement>>) -> ContainerStatement {
        ContainerStatement { contained_statements }
    }
}

impl Statement for ContainerStatement {
    fn assemble(
        &self,
        address: u16,
        label_map: &HashMap<String, u16>,
        _: &Backtrace,
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
