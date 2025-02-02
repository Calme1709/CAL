use std::collections::HashMap;

use logos::{Lexer, Logos};
use crate::{encode_signed_integer, encode_unsigned_integer, statements::{Add, Ascii, Block, Branch, Call, Halt, Load, LoadEffectiveAddress, LoadImmediate, Return, Sleep, Statement, StatementContainer, Store, Sub, Word}};

use super::tokens::{ Mnemonic, Token };

macro_rules! expect_token {
    ( $token_result:expr, $span:expr ) => {
        match $token_result {
            Some(Ok(token)) => token,
            Some(Err(e)) => return Err(AssemblerError { span: $span, error: e }),
            None => return Err(AssemblerError { span: $span, error: format!("Unexpected end of file") })
        }
    }
}

// TODO: Possibly implement a macro to support when there are multiple valid tokens (although it wouldn't make sense to return the underlying value then)
macro_rules! expect_token_of_type {
    ( $token_result:expr, $token_type:path, $span:expr ) => {
        match expect_token![$token_result, $span] {
            $token_type(value) => value,
            _ => return Err(AssemblerError { span: $span, error: format!("Unexpected token \"{:?}\", expected {}", $token_result.unwrap().unwrap(), stringify!($token_type)) })
        }
    }
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
            let token_or_none = self.lexer.next();

            if token_or_none.is_none() {
                break;
            }

            let token = expect_token!(token_or_none, self.lexer.span());

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
        let destination_register = expect_token_of_type!(self.lexer.next(), Token::Register, self.lexer.span());
        let source_register_zero = expect_token_of_type!(self.lexer.next(), Token::Register, self.lexer.span());

        let source_one_token = expect_token!(self.lexer.next(), self.lexer.span());

        let source_one_value = match source_one_token {
            Token::Register(source_register_one) => (1 << 5) | ((source_register_one) << 2),
            Token::NumericLiteral(numeric_literal) => encode_unsigned_integer!(numeric_literal, 5, self.lexer.span())?,
            _ => return Err(AssemblerError {
                span: self.lexer.span(),
                error: format!("Source one for add should be an IMM5 or a register - received {:?}", source_one_token)
            })
        };

        Ok(Add::new(destination_register, source_register_zero, source_one_value))
    }

    fn parse_sub_statement(&mut self) -> Result<Sub, AssemblerError> {
        let destination_register = expect_token_of_type!(self.lexer.next(), Token::Register, self.lexer.span());
        let source_register_zero = expect_token_of_type!(self.lexer.next(), Token::Register, self.lexer.span());

        let source_one_token = expect_token!(self.lexer.next(), self.lexer.span());

        let source_one_value = match source_one_token {
            Token::Register(source_register_one) => (1 << 5) | ((source_register_one) << 2),
            Token::NumericLiteral(numeric_literal) => encode_unsigned_integer!(numeric_literal, 5, self.lexer.span())?,
            _ => return Err(AssemblerError {
                span: self.lexer.span(),
                error: format!("Source one for sub should be an IMM5 or a register - received {:?}", source_one_token)
            })
        };

        Ok(Sub::new(destination_register, source_register_zero, source_one_value))
    }

    fn parse_load_effective_address_statement(&mut self) -> Result<LoadEffectiveAddress, AssemblerError> {
        let destination_register = expect_token_of_type!(self.lexer.next(), Token::Register, self.lexer.span());

        let offset_token = expect_token!(self.lexer.next(), self.lexer.span());

        match offset_token {
            Token::NumericLiteral(numeric_literal) => Ok(LoadEffectiveAddress::from_numeric_literal(destination_register, numeric_literal)),
            Token::Label(label) => Ok(LoadEffectiveAddress::from_label(destination_register, label)),
            _ => Err(AssemblerError{
                span: self.lexer.span(),
                error: format!("Unexpected token \"{:?}\", expected NumericLiteral or Label", offset_token)
            })
        }
    }

    fn parse_load_statement(&mut self) -> Result<Load, AssemblerError> {
        let destination_register = expect_token_of_type!(self.lexer.next(), Token::Register, self.lexer.span());
        let base_register = expect_token_of_type!(self.lexer.next(), Token::Register, self.lexer.span());
        let offset = expect_token_of_type!(self.lexer.next(), Token::NumericLiteral, self.lexer.span());

        Ok(Load::new(destination_register, base_register, offset))
    }

    fn parse_load_immediate_statement(&mut self) -> Result<LoadImmediate, AssemblerError> {
        let destination_register = expect_token_of_type!(self.lexer.next(), Token::Register, self.lexer.span());
        let numeric_literal = expect_token_of_type!(self.lexer.next(), Token::NumericLiteral, self.lexer.span());

        let encoded_numeric_literal = encode_unsigned_integer!(numeric_literal, 9, self.lexer.span())?;

        Ok(LoadImmediate::new(destination_register, encoded_numeric_literal))
    }

    fn parse_store_statement(&mut self) -> Result<Store, AssemblerError> {
        let base_register = expect_token_of_type!(self.lexer.next(), Token::Register, self.lexer.span());
        let offset = expect_token_of_type!(self.lexer.next(), Token::NumericLiteral, self.lexer.span());
        let source_register = expect_token_of_type!(self.lexer.next(), Token::Register, self.lexer.span());

        Ok(Store::new(base_register, offset, source_register))
    }

    fn parse_branch_statement(&mut self) -> Result<Branch, AssemblerError> {
        let conditions = expect_token_of_type!(self.lexer.next(), Token::BranchConditons, self.lexer.span()).bits();

        let destination_token = expect_token!(self.lexer.next(), self.lexer.span());

        match destination_token {
            Token::NumericLiteral(numeric_literal) => Ok(Branch::from_numeric_literal(conditions, numeric_literal)),
            Token::Label(label) => Ok(Branch::from_label(conditions, label)),
            _ => Err(AssemblerError{
                span: self.lexer.span(),
                error: format!("Unexpected token \"{:?}\", expected NumericLiteral or Label", destination_token)
            })
        }
    }

    fn parse_call_statement(&mut self) -> Result<Call, AssemblerError> {
        let token = expect_token!(self.lexer.next(), self.lexer.span());

        match token {
            Token::Register(base_register) => {
                let offset = expect_token_of_type!(self.lexer.next(), Token::NumericLiteral, self.lexer.span());
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
            _ => Err(AssemblerError {
                span: self.lexer.span(),
                error: format!("Unexpected token \"{:?}\", expected Register, NumericLiteral, or Label", token)
            })
        }
    }

    fn parse_return_statement(&mut self) -> Result<Return, AssemblerError> {
        Ok(Return::new())
    }

    fn parse_halt_statement(&mut self) -> Result<Halt, AssemblerError> {
        Ok(Halt::new())
    }

    fn parse_sleep_statement(&mut self) -> Result<Sleep, AssemblerError> {
        let duration = expect_token_of_type!(self.lexer.next(), Token::NumericLiteral, self.lexer.span());

        let encoded_duration = encode_unsigned_integer!(duration, 12, self.lexer.span())?;

        Ok(Sleep::new(encoded_duration))
    }

    fn parse_word_statement(&mut self) -> Result<Word, AssemblerError> {
        let numeric_literal = expect_token_of_type!(self.lexer.next(), Token::NumericLiteral, self.lexer.span());

        let encoded_value = encode_unsigned_integer!(numeric_literal, 16, self.lexer.span())?;

        Ok(Word::new(encoded_value))
    }

    fn parse_ascii_statement(&mut self) -> Result<Ascii, AssemblerError> {
        let string = expect_token_of_type!(self.lexer.next(), Token::String, self.lexer.span());

        Ok(Ascii::new(&string))
    }

    fn parse_block_statement(&mut self) -> Result<Block, AssemblerError> {
        let numeric_literal = expect_token_of_type!(self.lexer.next(), Token::NumericLiteral, self.lexer.span());

        let size = encode_unsigned_integer!(numeric_literal, 16, self.lexer.span())?;

        Ok(Block::new(size))
    }
}