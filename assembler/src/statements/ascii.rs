use std::{collections::HashMap, ops::Range};

use crate::assembler::AssemblerError;

use super::Statement;

pub struct Ascii {
    value: String,
}

impl Ascii {
    pub fn new(value: &str) -> Ascii {
        Ascii {
            value: value.to_owned(),
        }
    }
}

impl Statement for Ascii {
    fn assemble(&self, _: u16, _: &HashMap<String, u16>, _: &Range<usize>) -> Result<Vec<u16>, AssemblerError> {
        let mut out: Vec<u16> = self.value.as_bytes().iter().map(|byte| *byte as u16).collect();

        out.extend(vec![0 as u16]);

        return Ok(out);
    }

    fn width(&self) -> u16 {
        return self.value.len() as u16 + 1;
    }
}
