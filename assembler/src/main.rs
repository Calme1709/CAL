use std::fs;

use assembler::assemble;

mod assembler;
mod statements;
mod tokens;
mod utils;

fn main() {
    let input_path = std::env::args().nth(1).expect("No input path provided");
    let output_path = std::env::args().nth(2).expect("No output path provided");

    let resolved_input_path = std::path::Path::new(&input_path);

    let assembly_code = fs::read_to_string(&resolved_input_path).expect("Could not read input file");

    match assemble(&assembly_code) {
        Ok(machine_code) => {
            let mut bytes: Vec<u8> = Vec::new();

            for word in machine_code {
                bytes.push((word >> 8 & 0xFF) as u8);
                bytes.push((word & 0xFF) as u8)
            }

            fs::write(output_path, bytes).unwrap();
        }
        // TODO: Improve error print out
        Err(err) => print!("{} at {:?}", err.error, err.span),
    }
}
