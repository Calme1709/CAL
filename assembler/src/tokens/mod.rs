use logos::{Lexer, Logos};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mnemonic {
    Add,
    Halt
}

// TODO: Support other bases than 10
fn imm5_callback(lexer: &mut Lexer<Token>) -> Result<u8, String> {
    match lexer.slice()[1..].parse::<u8>() {
        Ok(value @ 0..31) => Ok(value),
        Ok(_) => Err(format!("Invalid value for Imm5 \"{}\", values must fall in the range 0 to 31", lexer.slice())),
        Err(_) => Err(format!("Failed to parse numeric literal {}", lexer.slice()))
    }
}

fn mnemonic_callback(lexer: &mut Lexer<Token>) -> Result<Mnemonic, String> {
    match lexer.slice() {
        "ADD" => Ok(Mnemonic::Add),
        "HLT" => Ok(Mnemonic::Halt),
        _ => Err(format!("Unrecognized mnemonic \"{}\"", lexer.slice()))
    }
}

fn register_callback(lexer: &mut Lexer<Token>) -> Option<u8> {
    Some(lexer.slice().chars().nth(1)? as u8 - 0x30)
}

#[derive(Logos, Clone, Copy, Debug, PartialEq)]
#[logos(skip r"[\s\r\n\f]+", error=String)]
pub enum Token {
    #[regex("#-?[0-9]+", imm5_callback)]
    Imm5(u8),

    #[regex("ADD|HLT", mnemonic_callback)]
    Mnemonic(Mnemonic),

    #[regex("R[0-7]", register_callback)]
    Register(u8)
}