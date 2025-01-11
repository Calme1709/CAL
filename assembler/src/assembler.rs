use logos::{Lexer, Logos};
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
    ( $token_result:expr, $token_type:ident, $span:expr ) => {
        match expect_token![$token_result, $span] {
            Token::$token_type(value) => value,
            // TODO: Print out expected token
            _ => return Err(AssemblerError { span: $span, error: format!("Unexpected token \"{:?}\"", $token_result.unwrap().unwrap()) })
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
        let mut out = Vec::new();

        loop {
            let token = self.lexer.next();

            if token.is_none() {
                break;
            }

            let mnemonic = expect_token_of_type!(token, Mnemonic, self.lexer.span());

            match mnemonic {
                Mnemonic::Add => out.push(self.assemble_add_statement()?),
                Mnemonic::Sub => out.push(self.assemble_sub_statement()?),
                Mnemonic::Halt => out.push(0b1100000000000000),
            }
        }

        return Ok(out)
    }

    fn assemble_add_statement(&mut self) -> Result<u16, AssemblerError> {
        let destination_register = expect_token_of_type!(self.lexer.next(), Register, self.lexer.span());
        let source_register_zero = expect_token_of_type!(self.lexer.next(), Register, self.lexer.span());

        let source_one_token = expect_token!(self.lexer.next(), self.lexer.span());

        let source_one_value = match source_one_token {
            Token::Register(source_register_one) => (1 << 5) | ((source_register_one) << 2),
            Token::Imm5(imm5) => imm5,
            _ => return Err(AssemblerError {
                span: self.lexer.span(),
                error: format!("Source one for add should be an IMM5 or a register - received {:?}", source_one_token)
            })
        };

        return Ok((0b0000 << 12) | (destination_register << 9) | (source_register_zero << 6) | source_one_value);
    }

    fn assemble_sub_statement(&mut self) -> Result<u16, AssemblerError> {
        let destination_register = expect_token_of_type!(self.lexer.next(), Register, self.lexer.span());
        let source_register_zero = expect_token_of_type!(self.lexer.next(), Register, self.lexer.span());

        let source_one_token = expect_token!(self.lexer.next(), self.lexer.span());

        let source_one_value = match source_one_token {
            Token::Register(source_register_one) => (1 << 5) | ((source_register_one) << 2),
            Token::Imm5(imm5) => imm5,
            _ => return Err(AssemblerError {
                span: self.lexer.span(),
                error: format!("Source one for sub should be an IMM5 or a register - received {:?}", source_one_token)
            })
        };

        return Ok((0b0001 << 12) | (destination_register << 9) | (source_register_zero << 6) | source_one_value);
    }
}