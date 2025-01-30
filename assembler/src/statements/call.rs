use std::{collections::HashMap, ops::Range};

use crate::{assembler::AssemblerError, utils::get_encoded_label_offset};

use super::Statement;

struct RegisterRelativeCallTypeParams {
    base_register: u16,
    encoded_offset: u16
}

enum PCRelativeCallTypeParams {
    Label(String),
    EncodedOffset(u16)
}

enum CallTypeParams {
    RegisterRelative(RegisterRelativeCallTypeParams),
    PCRelative(PCRelativeCallTypeParams)
}

pub struct Call {
    params: CallTypeParams,
}

impl Call {
    pub fn from_label(label: &str) -> Call {
        Call {
            params: CallTypeParams::PCRelative(
                PCRelativeCallTypeParams::Label(
                    label.to_owned()
                )
            )
        }
    }

    pub fn from_encoded_offset(encoded_offset: u16) -> Call {
        Call {
            params: CallTypeParams::PCRelative(
                PCRelativeCallTypeParams::EncodedOffset(encoded_offset)
            )
        }
    }

    pub fn from_register_and_offset(base_register: u16, encoded_offset: u16) -> Call {
        Call {
            params: CallTypeParams::RegisterRelative(
                RegisterRelativeCallTypeParams {
                    base_register,
                    encoded_offset
                }
            )
        }
    }
}

impl Statement for Call {
    fn assemble(&self, address: u16, label_map: &HashMap<String, u16>, span: &Range<usize>) -> Result<Vec<u16>, AssemblerError> {
        let result = match &self.params {
            CallTypeParams::RegisterRelative(params) => {
                (0b10100 << 11) | (params.base_register << 8) | params.encoded_offset
            },
            CallTypeParams::PCRelative(params) => match params {
                PCRelativeCallTypeParams::EncodedOffset(encoded_offset) => (0b10101 << 11) | encoded_offset,
                PCRelativeCallTypeParams::Label(label) => (0b10101 << 11) | get_encoded_label_offset(address + 1, &label, label_map, 11, span)?
            }
        };

        Ok(vec![result])
    }

    fn width(&self) -> u16 {
        return 1;
    }
}