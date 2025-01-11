use logos::{Lexer, Logos};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mnemonic {
    Add,
    Sub,
    Halt
}

fn imm5_callback(lexer: &mut Lexer<Token>)-> Result<u16, String> {
    immediate_callback(5)(lexer)
}

// TODO: Support other bases than 10
fn immediate_callback(bits: u32) -> impl Fn(&mut Lexer<Token>) -> Result<u16, String> {
    let max_value = (2 as u16).pow(bits) - 1;

    move |lexer: &mut Lexer<Token>| {
        match lexer.slice()[1..].parse::<u16>() {
            Ok(value) if value >= 0 && value <= max_value => Ok(value),
            Ok(_) => Err(format!("Invalid value for Imm{} \"{}\", values must fall in the range 0 to {}", bits, lexer.slice(), max_value)),
            Err(_) => Err(format!("Failed to parse numeric literal {}", lexer.slice()))
        }
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
    #[regex("#-?[0-9]+", imm5_callback)]
    Imm5(u16),

    #[regex("ADD|SUB|HLT", mnemonic_callback)]
    Mnemonic(Mnemonic),

    #[regex("R[0-7]", register_callback)]
    Register(u16),
}