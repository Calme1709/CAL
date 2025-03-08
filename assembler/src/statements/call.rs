use std::collections::HashMap;

use crate::assembler::{AssemblerError, Backtrace};

use super::Statement;

pub struct Call {
    label: String,
}

impl Call {
    pub fn new(label: &str) -> Call {
        Call {
            label: label.to_owned(),
        }
    }
}

impl Statement for Call {
    fn assemble(
        &self,
        _: u16,
        label_map: &HashMap<String, u16>,
        subroutine_lookup_table_entries: &Vec<String>,
        backtrace: &Backtrace,
    ) -> Result<Vec<u16>, AssemblerError> {
        if !label_map.contains_key(&self.label) {
            return Err(AssemblerError::new(
                format!("Unrecognized subroutine name: {}", self.label),
                backtrace.clone(),
            ));
        }

        let lookup_table_index = subroutine_lookup_table_entries
            .iter()
            .position(|label| *label == self.label)
            .unwrap() as u16;

        Ok(vec![(0b1010 << 12) | lookup_table_index])
    }

    fn width(&self) -> u16 {
        return 1;
    }
}
