use logos::{Lexer, Logos};
use shared::BranchConditions;

// TODO: Support other bases than 10
fn numeric_literal_callback(lexer: &mut Lexer<Token>) -> Result<i32, String> {
    match lexer.slice()[1..].parse::<i32>() {
        Ok(value) => Ok(value),
        Err(_) => Err(format!("Failed to parse numeric literal {}", lexer.slice())),
    }
}

fn identifier_callback(lexer: &mut Lexer<Token>) -> String {
    return lexer.slice().to_owned();
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

// Escape \n in user provided strings
fn string_callback(lexer: &mut Lexer<Token>) -> String {
    let slice = lexer.slice();

    str::replace(&slice[1..slice.len() - 1], r"\n", "\n")
}

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\r\n\f]+", error=String)]
pub enum Token {
    #[regex("//.+\n")]
    Comment,

    #[regex("#-?[0-9]+", numeric_literal_callback)]
    NumericLiteral(i32),

    #[regex("[A-Z0-9]+", identifier_callback)]
    Identifier(String),

    #[regex("R[0-7]", register_callback)]
    Register(u16),

    #[regex("nzp|nz|np|n|zp|z|p", branch_conditions_callback)]
    BranchConditons(BranchConditions),

    #[regex("\\.[A-z0-9]+", label_callback)]
    Label(String),

    #[regex(r#""(?:[^"]|\\")*""#, string_callback)]
    String(String),
}
