mod state;

use std::fs;
use state::State;

fn main() {
    let binary_path = std::env::args().nth(1).expect("No binary provided");

    let resolved_path = std::path::Path::new(&binary_path);

    let bytes = fs::read(&resolved_path).expect("Could not read file");

    let mut state = State::new();

    bytes.chunks(2).enumerate().for_each(|(i, chunk)| {
        println!("{:?}", chunk);
        let word = u16::from_be_bytes([chunk[0], chunk[1]]);
        state.memory[i] = word;
    });

    println!("{:?}", state);
}
