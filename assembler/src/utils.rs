use std::collections::HashMap;

pub fn encode_signed_integer(integer: i32, bits: u32) -> Result<u16, String> {
    let min_value = -(2 as i32).pow(bits - 1);
    let max_value = (2 as i32).pow(bits - 1) - 1;

    match integer >= min_value && integer <= max_value {
        true => Ok(integer as u16 & ((2 as u32).pow(bits) - 1) as u16),
        false => Err(format!(
            "Invalid value for i{} \"{}\", values should be in range {}-{}",
            bits, integer, min_value, max_value
        )),
    }
}

pub fn encode_unsigned_integer(integer: i32, bits: u32) -> Result<u16, String> {
    let min_value = 0;
    let max_value = (2 as i32).pow(bits) - 1;

    match integer >= min_value && integer <= max_value {
        true => Ok(integer as u16),
        false => Err(format!(
            "Invalid value for u{} \"{}\", values should be in range 0-{}",
            bits, integer, max_value
        )),
    }
}

pub fn get_encoded_label_offset(
    address: u16,
    label: &str,
    label_map: &HashMap<String, u16>,
    bits: u16,
) -> Result<u16, String> {
    match label_map.get(label) {
        Some(label_address) => {
            let offset = (*label_address as i32) - (address as i32);

            // TODO: Allow wrapping (e.g. 65535 is in range of 0 as -1)
            match encode_signed_integer((*label_address as i32) - (address as i32), bits as u32) {
                Ok(encoded_offset) => Ok(encoded_offset),
                Err(_) => Err(format!(
                    "Label {} out of range, requires offset of {} but must be within range -256..255",
                    label, offset
                )),
            }
        }
        None => return Err(format!("Unrecognized label {}", label)),
    }
}
