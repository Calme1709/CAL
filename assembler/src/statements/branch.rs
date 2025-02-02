use std::{collections::HashMap, ops::Range};

use crate::{assembler::AssemblerError, encode_signed_integer, utils::get_encoded_label_offset};

use super::Statement;

#[derive(Clone)]
enum LabelOrOffset {
    Label(String),
    Offset(i32),
}

pub struct Branch {
    conditions: u16,
    label_or_offset: LabelOrOffset,
}

impl Branch {
    pub fn from_numeric_literal(conditions: u16, offset: i32) -> Branch {
        Branch {
            conditions,
            label_or_offset: LabelOrOffset::Offset(offset),
        }
    }

    pub fn from_label(conditions: u16, label: String) -> Branch {
        Branch {
            conditions,
            label_or_offset: LabelOrOffset::Label(label),
        }
    }
}

impl Statement for Branch {
    fn assemble(
        &self,
        address: u16,
        label_map: &HashMap<String, u16>,
        span: &Range<usize>,
    ) -> Result<Vec<u16>, AssemblerError> {
        let encoded_offset = match self.label_or_offset.clone() {
            LabelOrOffset::Offset(offset) => encode_signed_integer!(offset, 9, span.to_owned())?,
            LabelOrOffset::Label(label) => get_encoded_label_offset(address + 1, &label, label_map, 9, span)?,
        };

        return Ok(vec![(0b1001 << 12) | (self.conditions << 9) | encoded_offset]);
    }

    fn width(&self) -> u16 {
        return 1;
    }
}
