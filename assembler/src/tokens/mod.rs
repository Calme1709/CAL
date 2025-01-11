use logos::{Lexer, Logos};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mnemonic {
    Add,
    Sub,
    Halt
}

// TODO: Support other bases than 10
fn numeric_literal_callback(lexer: &mut Lexer<Token>) -> Result<i32, String> {
    match lexer.slice()[1..].parse::<i32>() {
        Ok(value) => Ok(value),
        Err(_) => Err(format!("Failed to parse numeric literal {}", lexer.slice()))
    }
}

fn mnemonic_callback(lexer: &mut Lexer<Token>) -> Result<Mnemonic, String> {
    match lexer.slice() {
        "ADD" => Ok(Mnemonic::Add),
        "SUB" => Ok(Mnemonic::Sub),
        "HLT" => Ok(Mnemonic::Halt),
        _ => Err(format!("Unrecognized mnemonic \"{}\"", lexer.slice()))
    }
}

fn register_callback(lexer: &mut Lexer<Token>) -> Option<u16> {
    Some(lexer.slice().chars().nth(1)? as u16 - 0x30)
}

#[derive(Logos, Clone, Copy, Debug, PartialEq)]
#[logos(skip r"[\s\r\n\f]+", error=String)]
pub enum Token {
    #[regex("#-?[0-9]+", numeric_literal_callback)]
    NumericLiteral(i32),

    #[regex("ADD|SUB|HLT", mnemonic_callback)]
    Mnemonic(Mnemonic),

    #[regex("R[0-7]", register_callback)]
    Register(u16),
}