use std::collections::HashMap;

use crate::{
    encode_signed_integer, encode_unsigned_integer,
    statements::{
        Add, Ascii, Block, Branch, Call, Halt, Load, LoadEffectiveAddress, LoadImmediate, Return, Sleep, Statement,
        StatementContainer, Store, Sub, Word,
    },
};

use logos::Lexer;

use super::tokens::Token;

macro_rules! next_token {
    ( $lexer:expr ) => {
        match $lexer.next() {
            Some(Ok(token)) => Ok(token),
            Some(Err(e)) => Err(AssemblerError { span: $lexer.span(), error: e }),
            None => Err(AssemblerError { span: $lexer.span(), error: format!("Unexpected end of file") })
        }
    };
    ( $lexer:expr, $($token_type:path),+ ) => {
        match next_token!($lexer) {
            $( Ok(token @ $token_type(_) ) => Ok(token), )+
            Ok(token) => {
                let expected_tokens = vec![ $( stringify!($token_type), )+ ];

                let expected_tokens_str = match expected_tokens.len() {
                    1 => expected_tokens.get(0).unwrap().to_owned().to_owned(),
                    2 => format!("{} or {}", expected_tokens.get(0).unwrap(), expected_tokens.get(1).unwrap()),
                    _ => format!("{}, or {}", expected_tokens[0..(expected_tokens.len() - 1)].join(", "), expected_tokens.get(expected_tokens.len() - 1).unwrap())
                };

                Err(AssemblerError { span: $lexer.span(), error: format!("Unexpected token \"{:?}\", expected {}", token, expected_tokens_str) })
            },
            Err(err) => Err(err)
        }
    }
}

macro_rules! next_token_unwrapped {
    ( $lexer:expr, $token_type:path ) => {{
        let next_token_result = next_token!($lexer, $token_type);

        match next_token_result {
            Ok(token) => match token {
                $token_type(value) => Ok(value),
                _ => unreachable!(),
            },
            Err(err) => Err(err),
        }
    }};
}

pub struct AssemblerError {
    pub span: core::ops::Range<usize>,
    pub error: String,
}

pub fn assemble(source: &str) -> Result<Vec<u16>, AssemblerError> {
    let mut lexer = Lexer::new(source);

    let mut label_map: HashMap<String, u16> = HashMap::new();
    let mut statements: Vec<StatementContainer<dyn Statement>> = Vec::new();
    let mut label_address = 0;

    loop {
        let token = match lexer.next() {
            Some(Ok(token)) => token,
            Some(Err(e)) => {
                return Err(AssemblerError {
                    span: lexer.span(),
                    error: e,
                })
            }
            None => break,
        };

        // TODO: Disallow multiple consecutive labels
        match token {
            Token::Comment => {}
            Token::Label(label_name) => {
                if label_map.contains_key(&label_name) {
                    return Err(AssemblerError {
                        span: lexer.span(),
                        error: format!("Tried to redefine already existing label \"{}\"", label_name),
                    });
                }

                label_map.insert(label_name, label_address as u16);
            }
            Token::Identifier(identifier) => {
                let span_start = lexer.span().start;

                let statement: Box<dyn Statement> = match identifier.as_ref() {
                    // Instructions
                    "ADD" => Box::new(parse_add_statement(&mut lexer)?),
                    "SUB" => Box::new(parse_sub_statement(&mut lexer)?),
                    "LEA" => Box::new(parse_load_effective_address_statement(&mut lexer)?),
                    "LD" => Box::new(parse_load_statement(&mut lexer)?),
                    "LDI" => Box::new(parse_load_immediate_statement(&mut lexer)?),
                    "ST" => Box::new(parse_store_statement(&mut lexer)?),
                    "BR" => Box::new(parse_branch_statement(&mut lexer)?),
                    "CALL" => Box::new(parse_call_statement(&mut lexer)?),
                    "RET" => Box::new(parse_return_statement(&mut lexer)?),
                    "HLT" => Box::new(parse_halt_statement(&mut lexer)?),
                    "SLP" => Box::new(parse_sleep_statement(&mut lexer)?),

                    // Directives
                    "WORD" => Box::new(parse_word_statement(&mut lexer)?),
                    "ASCII" => Box::new(parse_ascii_statement(&mut lexer)?),
                    "BLK" => Box::new(parse_block_statement(&mut lexer)?),

                    _ => {
                        return Err(AssemblerError {
                            span: lexer.span(),
                            error: format!("Unrecognized identifier {}", identifier),
                        })
                    }
                };

                let statement_container = StatementContainer::new(statement, span_start..(lexer.span().end));

                label_address += statement_container.width();
                statements.push(statement_container);
            }
            _ => {
                return Err(AssemblerError {
                    span: lexer.span(),
                    error: format!("Unexpected token \"{:?}\", expected Label or Mnemonic", token),
                })
            }
        }
    }

    let mut out = Vec::new();
    let mut statement_address = 0;

    for statement in statements {
        out.append(statement.assemble(statement_address, &label_map)?.as_mut());
        statement_address += statement.width();
    }

    return Ok(out);
}

fn parse_add_statement(lexer: &mut Lexer<Token>) -> Result<Add, AssemblerError> {
    let destination_register = next_token_unwrapped!(lexer, Token::Register)?;
    let source_register_zero = next_token_unwrapped!(lexer, Token::Register)?;

    let source_one_value = match next_token!(lexer, Token::Register, Token::NumericLiteral)? {
        Token::Register(source_register_one) => (1 << 5) | ((source_register_one) << 2),
        Token::NumericLiteral(numeric_literal) => encode_unsigned_integer!(numeric_literal, 5, lexer.span())?,
        _ => unreachable!(),
    };

    Ok(Add::new(destination_register, source_register_zero, source_one_value))
}

fn parse_sub_statement(lexer: &mut Lexer<Token>) -> Result<Sub, AssemblerError> {
    let destination_register = next_token_unwrapped!(lexer, Token::Register)?;
    let source_register_zero = next_token_unwrapped!(lexer, Token::Register)?;

    let source_one_value = match next_token!(lexer, Token::Register, Token::NumericLiteral)? {
        Token::Register(source_register_one) => (1 << 5) | ((source_register_one) << 2),
        Token::NumericLiteral(numeric_literal) => encode_unsigned_integer!(numeric_literal, 5, lexer.span())?,
        _ => unreachable!(),
    };

    Ok(Sub::new(destination_register, source_register_zero, source_one_value))
}

fn parse_load_effective_address_statement(lexer: &mut Lexer<Token>) -> Result<LoadEffectiveAddress, AssemblerError> {
    let destination_register = next_token_unwrapped!(lexer, Token::Register)?;

    match next_token!(lexer, Token::NumericLiteral, Token::Label)? {
        Token::NumericLiteral(numeric_literal) => Ok(LoadEffectiveAddress::from_numeric_literal(
            destination_register,
            numeric_literal,
        )),
        Token::Label(label) => Ok(LoadEffectiveAddress::from_label(destination_register, label)),
        _ => unreachable!(),
    }
}

fn parse_load_statement(lexer: &mut Lexer<Token>) -> Result<Load, AssemblerError> {
    let destination_register = next_token_unwrapped!(lexer, Token::Register)?;
    let base_register = next_token_unwrapped!(lexer, Token::Register)?;
    let offset = next_token_unwrapped!(lexer, Token::NumericLiteral)?;

    Ok(Load::new(destination_register, base_register, offset))
}

fn parse_load_immediate_statement(lexer: &mut Lexer<Token>) -> Result<LoadImmediate, AssemblerError> {
    let destination_register = next_token_unwrapped!(lexer, Token::Register)?;
    let numeric_literal = next_token_unwrapped!(lexer, Token::NumericLiteral)?;

    let encoded_numeric_literal = encode_unsigned_integer!(numeric_literal, 9, lexer.span())?;

    Ok(LoadImmediate::new(destination_register, encoded_numeric_literal))
}

fn parse_store_statement(lexer: &mut Lexer<Token>) -> Result<Store, AssemblerError> {
    let base_register = next_token_unwrapped!(lexer, Token::Register)?;
    let offset = next_token_unwrapped!(lexer, Token::NumericLiteral)?;
    let source_register = next_token_unwrapped!(lexer, Token::Register)?;

    Ok(Store::new(base_register, offset, source_register))
}

fn parse_branch_statement(lexer: &mut Lexer<Token>) -> Result<Branch, AssemblerError> {
    let conditions = next_token_unwrapped!(lexer, Token::BranchConditons)?.bits();

    match next_token!(lexer, Token::NumericLiteral, Token::Label)? {
        Token::NumericLiteral(numeric_literal) => Ok(Branch::from_numeric_literal(conditions, numeric_literal)),
        Token::Label(label) => Ok(Branch::from_label(conditions, label)),
        _ => unreachable!(),
    }
}

fn parse_call_statement(lexer: &mut Lexer<Token>) -> Result<Call, AssemblerError> {
    match next_token!(lexer, Token::Register, Token::NumericLiteral, Token::Label)? {
        Token::Register(base_register) => {
            let offset = next_token_unwrapped!(lexer, Token::NumericLiteral)?;
            let encoded_offset = encode_signed_integer!(offset, 8, lexer.span())?;

            Ok(Call::from_register_and_offset(base_register, encoded_offset))
        }
        Token::NumericLiteral(offset) => {
            let encoded_offset = encode_signed_integer!(offset, 11, lexer.span())?;

            Ok(Call::from_encoded_offset(encoded_offset))
        }
        Token::Label(label) => Ok(Call::from_label(&label)),
        _ => unreachable!(),
    }
}

fn parse_return_statement<'a>(_: &mut Lexer<'a, Token>) -> Result<Return, AssemblerError> {
    Ok(Return::new())
}

fn parse_halt_statement<'a>(_: &mut Lexer<'a, Token>) -> Result<Halt, AssemblerError> {
    Ok(Halt::new())
}

fn parse_sleep_statement(lexer: &mut Lexer<Token>) -> Result<Sleep, AssemblerError> {
    let duration = next_token_unwrapped!(lexer, Token::NumericLiteral)?;

    let encoded_duration = encode_unsigned_integer!(duration, 12, lexer.span())?;

    Ok(Sleep::new(encoded_duration))
}

fn parse_word_statement(lexer: &mut Lexer<Token>) -> Result<Word, AssemblerError> {
    let numeric_literal = next_token_unwrapped!(lexer, Token::NumericLiteral)?;

    let encoded_value = encode_unsigned_integer!(numeric_literal, 16, lexer.span())?;

    Ok(Word::new(encoded_value))
}

fn parse_ascii_statement(lexer: &mut Lexer<Token>) -> Result<Ascii, AssemblerError> {
    let string = next_token_unwrapped!(lexer, Token::String)?;

    Ok(Ascii::new(&string))
}

fn parse_block_statement(lexer: &mut Lexer<Token>) -> Result<Block, AssemblerError> {
    let numeric_literal = next_token_unwrapped!(lexer, Token::NumericLiteral)?;

    let size = encode_unsigned_integer!(numeric_literal, 16, lexer.span())?;

    Ok(Block::new(size))
}
