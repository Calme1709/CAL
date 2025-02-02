mod instructions;
mod state;
mod utils;

use nix::fcntl::{fcntl, FcntlArg, OFlag};
use nix::unistd::read;
use state::State;
use std::fs;

fn main() {
    let binary_path = std::env::args().nth(1).expect("No binary provided");

    let resolved_path = std::path::Path::new(&binary_path);

    let bytes = fs::read(&resolved_path).expect("Could not read file");

    let mut state = State::new();

    bytes.chunks(2).enumerate().for_each(|(i, chunk)| {
        let word = u16::from_be_bytes([chunk[0], chunk[1]]);
        state.memory[i] = word;
    });

    state = run_program(state);

    println!("\n{:?}", state);
}

fn run_program(mut state: State) -> State {
    let fd = 0;
    let flags = fcntl(fd, FcntlArg::F_GETFL).expect("Failed to get flags");

    fcntl(
        fd,
        FcntlArg::F_SETFL(OFlag::from_bits_truncate(flags) | OFlag::O_NONBLOCK),
    )
    .expect("Failed to set non-blocking mode");

    while !state.halt {
        let mut stdin_buffer = [0u8; 1024];

        match read(fd, &mut stdin_buffer) {
            Ok(_) => state.stdin.append(&mut stdin_buffer.to_vec()),
            Err(_) => {}
        }

        let instruction = instructions::from_machine_code(state.memory[state.pc as usize]);

        // println!("{:?}", instruction);

        instruction.execute(&mut state);

        state.pc = state.pc.wrapping_add(1);
    }

    return state;
}
