use logos::{Lexer, Logos};
use shared::BranchConditions;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mnemonic {
    Add,
    Sub,
    Branch,
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
        "BR" => Ok(Mnemonic::Branch),
        "HLT" => Ok(Mnemonic::Halt),
        _ => Err(format!("Unrecognized mnemonic \"{}\"", lexer.slice()))
    }
}

fn register_callback(lexer: &mut Lexer<Token>) -> Option<u16> {
    Some(lexer.slice().chars().nth(1)? as u16 - 0x30)
}

fn branch_conditions_callback(lexer: &mut Lexer<Token>) -> BranchConditions {
    let mut out = BranchConditions::empty();

    if lexer.slice().contains('n') {
        out |= BranchConditions::NEGATIVE;
    }

    if lexer.slice().contains('z') {
        out |= BranchConditions::ZERO;
    }

    if lexer.slice().contains('p') {
        out |= BranchConditions::POSITIVE;
    }

    return out;
}

fn label_callback(lexer: &mut Lexer<Token>) -> String {
    lexer.slice()[1..].to_owned()
}

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\r\n\f]+", error=String)]
pub enum Token {
    #[regex("//.+\n")]
    Comment,

    #[regex("#-?[0-9]+", numeric_literal_callback)]
    NumericLiteral(i32),

    #[regex("ADD|SUB|BR|HLT", mnemonic_callback)]
    Mnemonic(Mnemonic),

    #[regex("R[0-7]", register_callback)]
    Register(u16),

    #[regex("nzp|nz|n|zp|z|p", branch_conditions_callback)]
    BranchConditons(BranchConditions),

    #[regex("\\.[A-z0-9]+", label_callback)]
    Label(String)
}