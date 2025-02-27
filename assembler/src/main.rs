use std::{
    fs,
    path::{absolute, Path},
};

use assembler::assemble;

mod assembler;
mod statements;
mod tokens;
mod utils;

fn main() {
    let input_path = std::env::args().nth(1).expect("No input path provided");
    let output_path = std::env::args().nth(2).expect("No output path provided");

    let absolute_input_path = absolute(Path::new(input_path.as_str()))
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    match assemble(absolute_input_path) {
        Ok(machine_code) => {
            let mut bytes: Vec<u8> = Vec::new();

            for word in machine_code {
                bytes.push((word >> 8 & 0xFF) as u8);
                bytes.push((word & 0xFF) as u8)
            }

            fs::write(output_path, bytes).unwrap();
        }
        // TODO: Improve error print out
        Err(err) => print!("{}", err),
    }
}
