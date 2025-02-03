use std::collections::HashMap;

use crate::{
    assembler::{AssemblerError, Backtrace},
    utils::{encode_signed_integer, get_encoded_label_offset},
};

use super::Statement;

#[derive(Clone)]
enum LabelOrOffset {
    Label(String),
    Offset(i32),
}

pub struct LoadEffectiveAddress {
    destination_register: u16,
    label_or_offset: LabelOrOffset,
}

impl LoadEffectiveAddress {
    pub fn from_numeric_literal(destination_register: u16, offset: i32) -> LoadEffectiveAddress {
        LoadEffectiveAddress {
            destination_register,
            label_or_offset: LabelOrOffset::Offset(offset),
        }
    }

    pub fn from_label(destination_register: u16, label: String) -> LoadEffectiveAddress {
        LoadEffectiveAddress {
            destination_register,
            label_or_offset: LabelOrOffset::Label(label),
        }
    }
}

impl Statement for LoadEffectiveAddress {
    fn assemble(
        &self,
        address: u16,
        label_map: &HashMap<String, u16>,
        backtrace: &Backtrace,
    ) -> Result<Vec<u16>, AssemblerError> {
        let encoded_offset_result = match self.label_or_offset.clone() {
            LabelOrOffset::Offset(offset) => encode_signed_integer(offset, 9),
            LabelOrOffset::Label(label) => get_encoded_label_offset(address, &label, label_map, 9),
        };

        let encoded_offset = match encoded_offset_result {
            Ok(value) => value,
            Err(e) => return Err(AssemblerError::new(e, backtrace.clone())),
        };

        return Ok(vec![(0b0101 << 12) | (self.destination_register << 9) | encoded_offset]);
    }

    fn width(&self) -> u16 {
        return 1;
    }
}
