use std::collections::HashMap;

use logos::{Lexer, Logos};
use crate::{encode_signed_integer, encode_unsigned_integer, statements::{Add, Ascii, Block, Branch, Call, Halt, Load, LoadEffectiveAddress, LoadImmediate, Return, Sleep, Statement, StatementContainer, Store, Sub, Word}};

use super::tokens::{ Mnemonic, Token };

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
    ( $lexer:expr, $token_type:path ) => { {
        let next_token_result = next_token!($lexer, $token_type);

        match next_token_result {
            Ok(token) => match token {
                $token_type(value) => Ok(value),
                _ => unreachable!()
            },
            Err(err) => Err(err)
        }
    } }
}

pub struct Assembler<'a> {
    lexer: Lexer<'a, Token>,
}

pub struct AssemblerError {
    pub span: core::ops::Range<usize>,
    pub error: String
}

impl Assembler<'_> {
    pub fn new<'a>(source: &'a String) -> Assembler<'a> {
        Assembler {
            lexer: Token::lexer(source.as_str())
        }
    }

    pub fn assemble(&mut self) -> Result<Vec<u16>, AssemblerError> {
        let mut label_map: HashMap<String, u16> = HashMap::new();
        let mut statements: Vec<StatementContainer<dyn Statement>> = Vec::new();
        let mut label_address = 0;

        loop {
            let token = match self.lexer.next() {
                Some(Ok(token )) => token,
                Some(Err(e)) => return Err(AssemblerError { span: self.lexer.span(), error: e }),
                None => break
            };

            // TODO: Disallow multiple consecutive labels
            match token {
                Token::Comment => {},
                Token::Label(label_name) => {
                    if label_map.contains_key(&label_name) {
                        return Err(AssemblerError {
                            span: self.lexer.span(),
                            error: format!("Tried to redefine already existing label \"{}\"", label_name)
                        });
                    }

                    label_map.insert(label_name, label_address as u16);
                },
                Token::Mnemonic(mnemonic) => {
                    let span_start = self.lexer.span().start;

                    let statement: Box<dyn Statement> = match mnemonic {
                        Mnemonic::Add => Box::new(self.parse_add_statement()?),
                        Mnemonic::Sub => Box::new(self.parse_sub_statement()?),
                        Mnemonic::LoadEffectiveAddress => Box::new(self.parse_load_effective_address_statement()?),
                        Mnemonic::Load => Box::new(self.parse_load_statement()?),
                        Mnemonic::LoadImmediate => Box::new(self.parse_load_immediate_statement()?),
                        Mnemonic::Store => Box::new(self.parse_store_statement()?),
                        Mnemonic::Branch => Box::new(self.parse_branch_statement()?),
                        Mnemonic::Call => Box::new(self.parse_call_statement()?),
                        Mnemonic::Return => Box::new(self.parse_return_statement()?),
                        Mnemonic::Halt => Box::new(self.parse_halt_statement()?),
                        Mnemonic::Sleep => Box::new(self.parse_sleep_statement()?),
                        Mnemonic::Word => Box::new(self.parse_word_statement()?),
                        Mnemonic::Ascii => Box::new(self.parse_ascii_statement()?),
                        Mnemonic::Block => Box::new(self.parse_block_statement()?)
                    };

                    let statement_container = StatementContainer::new(statement, span_start..(self.lexer.span().end));

                    label_address += statement_container.width();
                    statements.push(statement_container);
                },
                _ => return Err(AssemblerError { span: self.lexer.span(), error: format!("Unexpected token \"{:?}\", expected Label or Mnemonic", token) })
            }
        }

        let mut out = Vec::new();
        let mut statement_address = 0;

        for statement in statements {
            out.append(statement.assemble(statement_address, &label_map)?.as_mut());
            statement_address += statement.width();
        }

        return Ok(out)
    }

    fn parse_add_statement(&mut self) -> Result<Add, AssemblerError> {
        let destination_register = next_token_unwrapped!(self.lexer, Token::Register)?;
        let source_register_zero = next_token_unwrapped!(self.lexer, Token::Register)?;

        let source_one_value = match next_token!(self.lexer, Token::Register, Token::NumericLiteral)? {
            Token::Register(source_register_one) => (1 << 5) | ((source_register_one) << 2),
            Token::NumericLiteral(numeric_literal) => encode_unsigned_integer!(numeric_literal, 5, self.lexer.span())?,
            _ => unreachable!()
        };

        Ok(Add::new(destination_register, source_register_zero, source_one_value))
    }

    fn parse_sub_statement(&mut self) -> Result<Sub, AssemblerError> {
        let destination_register = next_token_unwrapped!(self.lexer, Token::Register)?;
        let source_register_zero = next_token_unwrapped!(self.lexer, Token::Register)?;

        let source_one_value = match next_token!(self.lexer, Token::Register, Token::NumericLiteral)? {
            Token::Register(source_register_one) => (1 << 5) | ((source_register_one) << 2),
            Token::NumericLiteral(numeric_literal) => encode_unsigned_integer!(numeric_literal, 5, self.lexer.span())?,
            _ => unreachable!()
        };

        Ok(Sub::new(destination_register, source_register_zero, source_one_value))
    }

    fn parse_load_effective_address_statement(&mut self) -> Result<LoadEffectiveAddress, AssemblerError> {
        let destination_register = next_token_unwrapped!(self.lexer, Token::Register)?;

        match next_token!(self.lexer, Token::NumericLiteral, Token::Label)? {
            Token::NumericLiteral(numeric_literal) => Ok(LoadEffectiveAddress::from_numeric_literal(destination_register, numeric_literal)),
            Token::Label(label) => Ok(LoadEffectiveAddress::from_label(destination_register, label)),
            _ => unreachable!()
        }
    }

    fn parse_load_statement(&mut self) -> Result<Load, AssemblerError> {
        let destination_register = next_token_unwrapped!(self.lexer, Token::Register)?;
        let base_register = next_token_unwrapped!(self.lexer, Token::Register)?;
        let offset = next_token_unwrapped!(self.lexer, Token::NumericLiteral)?;

        Ok(Load::new(destination_register, base_register, offset))
    }

    fn parse_load_immediate_statement(&mut self) -> Result<LoadImmediate, AssemblerError> {
        let destination_register = next_token_unwrapped!(self.lexer, Token::Register)?;
        let numeric_literal = next_token_unwrapped!(self.lexer, Token::NumericLiteral)?;

        let encoded_numeric_literal = encode_unsigned_integer!(numeric_literal, 9, self.lexer.span())?;

        Ok(LoadImmediate::new(destination_register, encoded_numeric_literal))
    }

    fn parse_store_statement(&mut self) -> Result<Store, AssemblerError> {
        let base_register = next_token_unwrapped!(self.lexer, Token::Register)?;
        let offset = next_token_unwrapped!(self.lexer, Token::NumericLiteral)?;
        let source_register = next_token_unwrapped!(self.lexer, Token::Register)?;

        Ok(Store::new(base_register, offset, source_register))
    }

    fn parse_branch_statement(&mut self) -> Result<Branch, AssemblerError> {
        let conditions = next_token_unwrapped!(self.lexer, Token::BranchConditons)?.bits();

        match next_token!(self.lexer, Token::NumericLiteral, Token::Label)? {
            Token::NumericLiteral(numeric_literal) => Ok(Branch::from_numeric_literal(conditions, numeric_literal)),
            Token::Label(label) => Ok(Branch::from_label(conditions, label)),
            _ => unreachable!()
        }
    }

    fn parse_call_statement(&mut self) -> Result<Call, AssemblerError> {
        match next_token!(self.lexer, Token::Register, Token::NumericLiteral, Token::Label)? {
            Token::Register(base_register) => {
                let offset = next_token_unwrapped!(self.lexer, Token::NumericLiteral)?;
                let encoded_offset = encode_signed_integer!(offset, 8, self.lexer.span())?;

                Ok(Call::from_register_and_offset(base_register, encoded_offset))
            },
            Token::NumericLiteral(offset) => {
                let encoded_offset = encode_signed_integer!(offset, 11, self.lexer.span())?;

                Ok(Call::from_encoded_offset(encoded_offset))
            },
            Token::Label(label) => {
                Ok(Call::from_label(&label))
            },
            _ => unreachable!()
        }
    }

    fn parse_return_statement(&mut self) -> Result<Return, AssemblerError> {
        Ok(Return::new())
    }

    fn parse_halt_statement(&mut self) -> Result<Halt, AssemblerError> {
        Ok(Halt::new())
    }

    fn parse_sleep_statement(&mut self) -> Result<Sleep, AssemblerError> {
        let duration = next_token_unwrapped!(self.lexer, Token::NumericLiteral)?;

        let encoded_duration = encode_unsigned_integer!(duration, 12, self.lexer.span())?;

        Ok(Sleep::new(encoded_duration))
    }

    fn parse_word_statement(&mut self) -> Result<Word, AssemblerError> {
        let numeric_literal = next_token_unwrapped!(self.lexer, Token::NumericLiteral)?;

        let encoded_value = encode_unsigned_integer!(numeric_literal, 16, self.lexer.span())?;

        Ok(Word::new(encoded_value))
    }

    fn parse_ascii_statement(&mut self) -> Result<Ascii, AssemblerError> {
        let string = next_token_unwrapped!(self.lexer, Token::String)?;

        Ok(Ascii::new(&string))
    }

    fn parse_block_statement(&mut self) -> Result<Block, AssemblerError> {
        let numeric_literal = next_token_unwrapped!(self.lexer, Token::NumericLiteral)?;

        let size = encode_unsigned_integer!(numeric_literal, 16, self.lexer.span())?;

        Ok(Block::new(size))
    }
}