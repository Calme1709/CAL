use std::{collections::HashMap, ops::Range};

use crate::{assembler::AssemblerError, encode_signed_integer};

use super::Statement;

#[derive(Clone)]
enum LabelOrOffset {
    Label(String),
    Offset(i32),
}

pub struct Branch {
    conditions: u16,
    label_or_offset: LabelOrOffset
}

impl Branch {
    pub fn from_numeric_literal(conditions: u16, offset: i32) -> Branch {
        Branch {
            conditions,
            label_or_offset: LabelOrOffset::Offset(offset)
        }
    }

    pub fn from_label(conditions: u16, label: String) -> Branch {
        Branch {
            conditions,
            label_or_offset: LabelOrOffset::Label(label)
        }
    }
}

impl Statement for Branch {
    fn assemble(&self, address: u16, label_map: &HashMap<String, u16>, span: &Range<usize>) -> Result<Vec<u16>, AssemblerError> {
        let encoded_offset = match self.label_or_offset.clone() {
            LabelOrOffset::Offset(offset) => encode_signed_integer!(offset, 9, span.to_owned())?,
            LabelOrOffset::Label(label) => match label_map.get(&label) {
                Some(label_address) => {
                    let offset = (*label_address as i32) - (address as i32 + 1);

                    // TODO: Allow wrapping (e.g. 65535 is in range of 0 as -1)
                    match encode_signed_integer!((*label_address as i32) - (address as i32 + 1), 9, span.clone()) {
                        Ok(encoded_offset) => encoded_offset,
                        Err(_) => return Err(AssemblerError {
                            span: span.clone(),
                            error: format!("Label {} out of range, requires offset of {} but must be within range -256..255", label, offset)
                        })
                    }                    
                },
                None => return Err(AssemblerError {
                    span: span.clone(),
                    error: format!("Unrecognized label {}", label)
                })
            }
        };

        return Ok(vec![(0b1001 << 12) | (self.conditions << 9) | encoded_offset]);
    }

    fn width(&self) -> u16 {
        return 1;
    }
}