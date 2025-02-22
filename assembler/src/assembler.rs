use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FormatResult},
    fs,
    ops::Range,
    path::Path,
};

use crate::{
    statements::{
        Add, Ascii, Block, Branch, Call, Halt, Load, LoadEffectiveAddress, LoadImmediate, MacroInvocationStatement,
        Return, Sleep, Statement, StatementContainer, Store, Sub, Word,
    },
    utils::{encode_signed_integer, encode_unsigned_integer},
};

use logos::Lexer;

use super::tokens::Token;

macro_rules! next_token {
    ( $lexer:expr, $parsing_context:expr ) => {
        match $lexer.next() {
            Some(Ok(token)) => Ok(token),
            Some(Err(e)) => Err(
                AssemblerError::new(
                    format!("Lexer error: {}", e),
                    $parsing_context.get_backtrace($lexer.span())
                )
            ),
            None => Err(
                AssemblerError::new(
                    format!("Unexpected end of file"),
                    $parsing_context.get_backtrace($lexer.span())
                )
            )
        }
    };
    ( $lexer:expr, $parsing_context:expr, $($token_type:path),+ ) => {
        match next_token!($lexer, $parsing_context) {
            $( Ok(token @ $token_type(_) ) => Ok(token), )+
            Ok(token) => {
                let expected_tokens = vec![ $( stringify!($token_type), )+ ];

                let expected_tokens_str = match expected_tokens.len() {
                    1 => expected_tokens.get(0).unwrap().to_owned().to_owned(),
                    2 => format!("{} or {}", expected_tokens.get(0).unwrap(), expected_tokens.get(1).unwrap()),
                    _ => format!("{}, or {}", expected_tokens[0..(expected_tokens.len() - 1)].join(", "), expected_tokens.get(expected_tokens.len() - 1).unwrap())
                };

                Err(
                    AssemblerError::new(
                        format!("Unexpected token \"{:?}\", expected {}", token, expected_tokens_str),
                        $parsing_context.get_backtrace($lexer.span())
                    )
                )
            },
            Err(err) => Err(err)
        }
    }
}

macro_rules! next_token_unwrapped {
    ( $lexer:expr, $parsing_context:expr, $token_type:path ) => {{
        let next_token_result = next_token!($lexer, $parsing_context, $token_type);

        match next_token_result {
            Ok(token) => match token {
                $token_type(value) => Ok(value),
                _ => unreachable!(),
            },
            Err(err) => Err(err),
        }
    }};
}

#[derive(Clone)]
pub struct SourceLocation {
    file: String,
    character_span: Range<usize>,
}

impl PartialEq for SourceLocation {
    fn eq(&self, other: &Self) -> bool {
        return self.file == other.file && self.character_span == other.character_span;
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

impl SourceLocation {
    pub fn as_human_readable_string(&self) -> String {
        let source = fs::read_to_string(Path::new(&self.file)).unwrap();

        let lines: Vec<&str> = source.split("\n").collect();

        let mut line_counter = 0;
        let mut char_counter = 0;

        for line in lines {
            if char_counter + line.len() >= self.character_span.start {
                return format!(
                    "{}:{}:{}",
                    self.file,
                    line_counter + 1,
                    (self.character_span.start - char_counter) + 1
                );
            }

            line_counter += 1;
            char_counter += line.len() + 1;
        }

        unreachable!();
    }
}

impl SourceLocation {
    pub fn new(file: String, character_span: Range<usize>) -> SourceLocation {
        SourceLocation { file, character_span }
    }
}

// A backtrace is a vector of source locations (deepest last)
pub type Backtrace = Vec<SourceLocation>;

pub struct ParsingContext {
    file: String,
    global_offset: usize,
    backtrace: Backtrace,
}

impl ParsingContext {
    pub fn new(file: String, global_offset: usize, backtrace: Backtrace) -> ParsingContext {
        ParsingContext {
            file,
            global_offset,
            backtrace,
        }
    }

    pub fn get_backtrace(&self, local_span: Range<usize>) -> Backtrace {
        let mut current_backtrace = self.backtrace.clone();

        let current_source_location = SourceLocation::new(
            self.file.clone(),
            (local_span.start + self.global_offset)..(local_span.end + self.global_offset),
        );

        current_backtrace.push(current_source_location);

        current_backtrace
    }
}

struct Macro {
    source: String,
    number_of_parameters: usize,
    definition_file: String,
    definition_offset: usize,
}

pub struct AssemblerError {
    pub backtrace: Backtrace,
    pub error: String,
}

impl AssemblerError {
    pub fn new(error: String, backtrace: Backtrace) -> AssemblerError {
        AssemblerError { error, backtrace }
    }
}

impl Display for AssemblerError {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        writeln!(f, "{}", self.error)?;

        let mut reversed_backtrace = self.backtrace.clone();

        reversed_backtrace.reverse();

        for backtrace_entry in reversed_backtrace {
            writeln!(f, "\t{}", backtrace_entry.as_human_readable_string())?;
        }

        Ok(())
    }
}

pub fn assemble(source: &str, parsing_context: &ParsingContext) -> Result<Vec<u16>, AssemblerError> {
    let mut lexer = Lexer::new(source);

    let mut label_map: HashMap<String, u16> = HashMap::new();
    let mut statements: Vec<StatementContainer<dyn Statement>> = Vec::new();
    let mut label_address = 0;
    let mut macros: HashMap<String, Macro> = HashMap::new();

    loop {
        let token = match lexer.next() {
            Some(Ok(token)) => token,
            Some(Err(e)) => {
                return Err(AssemblerError::new(
                    format!("Lexer error: {}", e),
                    parsing_context.get_backtrace(lexer.span()),
                ))
            }
            None => break,
        };

        // TODO: Disallow multiple consecutive labels
        match token {
            Token::Comment => {}
            Token::Label(label_name) => {
                if label_map.contains_key(&label_name) {
                    return Err(AssemblerError::new(
                        format!("Tried to redefine already existing label \"{}\"", label_name),
                        parsing_context.get_backtrace(lexer.span()),
                    ));
                }

                label_map.insert(label_name, label_address as u16);
            }
            Token::Identifier(identifier) => {
                let statement_container = parse_statement(identifier, &mut lexer, &macros, &parsing_context)?;

                label_address += statement_container.width();
                statements.push(statement_container);
            }
            Token::MacroStart => {
                let macro_identifier = next_token_unwrapped!(lexer, parsing_context, Token::Identifier)?;

                let number_of_params = next_token_unwrapped!(lexer, parsing_context, Token::NumericLiteral)?;

                if number_of_params < 0 {
                    return Err(AssemblerError::new(
                        format!("Number of arguments for a macro must be greater than zero"),
                        parsing_context.get_backtrace(lexer.span()),
                    ));
                }

                let mut token = next_token!(lexer, parsing_context)?;

                let span_start = lexer.span().start;
                let mut span_end = span_start;

                loop {
                    if let Token::MacroEnd = token {
                        break;
                    }

                    if let Token::MacroParameter(parameter) = token {
                        if parameter >= (number_of_params as usize) {
                            return Err(AssemblerError::new(
                                format!("Parameter out of valid range (0-{}): ${}", number_of_params, parameter),
                                parsing_context.get_backtrace(lexer.span()),
                            ));
                        }
                    }

                    span_end = lexer.span().end;

                    token = next_token!(lexer, parsing_context)?;
                }

                macros.insert(
                    macro_identifier,
                    Macro {
                        source: source[span_start..span_end].to_owned(),
                        number_of_parameters: number_of_params as usize,
                        definition_file: parsing_context.file.clone(),
                        definition_offset: span_start,
                    },
                );
            }
            _ => {
                return Err(AssemblerError::new(
                    format!("Unexpected token \"{:?}\", expected Label or Mnemonic", token),
                    parsing_context.get_backtrace(lexer.span()),
                ))
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

fn parse_statement(
    identifier: String,
    mut lexer: &mut Lexer<Token>,
    macros: &HashMap<String, Macro>,
    parsing_context: &ParsingContext,
) -> Result<StatementContainer<dyn Statement>, AssemblerError> {
    let span_start = lexer.span().start;

    let statement: Box<dyn Statement> = match identifier.as_ref() {
        // Instructions
        "ADD" => Box::new(parse_add_statement(&mut lexer, parsing_context)?),
        "SUB" => Box::new(parse_sub_statement(&mut lexer, parsing_context)?),
        "LEA" => Box::new(parse_load_effective_address_statement(&mut lexer, parsing_context)?),
        "LD" => Box::new(parse_load_statement(&mut lexer, parsing_context)?),
        "LDI" => Box::new(parse_load_immediate_statement(&mut lexer, parsing_context)?),
        "ST" => Box::new(parse_store_statement(&mut lexer, parsing_context)?),
        "BR" => Box::new(parse_branch_statement(&mut lexer, parsing_context)?),
        "CALL" => Box::new(parse_call_statement(&mut lexer, parsing_context)?),
        "RET" => Box::new(parse_return_statement(&mut lexer, parsing_context)?),
        "HLT" => Box::new(parse_halt_statement(&mut lexer, parsing_context)?),
        "SLP" => Box::new(parse_sleep_statement(&mut lexer, parsing_context)?),

        // Directives
        "WORD" => Box::new(parse_word_statement(&mut lexer, parsing_context)?),
        "ASCII" => Box::new(parse_ascii_statement(&mut lexer, parsing_context)?),
        "BLK" => Box::new(parse_block_statement(&mut lexer, parsing_context)?),

        _ => match macros.get(&identifier) {
            Some(r#macro) => {
                let source_location = parsing_context.get_backtrace(lexer.span()).last().unwrap().to_owned();

                for backtrace_entry in parsing_context.backtrace.clone() {
                    if backtrace_entry == source_location {
                        return Err(AssemblerError::new(
                            "Detected recurisve invocation of macro".to_string(),
                            parsing_context.get_backtrace(lexer.span()),
                        ));
                    }
                }

                let macro_parsing_context = ParsingContext::new(
                    r#macro.definition_file.clone(),
                    r#macro.definition_offset,
                    parsing_context.get_backtrace(lexer.span()),
                );

                let mut macro_source = r#macro.source.to_owned();

                for i in 0..r#macro.number_of_parameters {
                    next_token!(lexer, parsing_context)?;

                    macro_source = macro_source.replace(&format!("${}", i), lexer.slice());
                }

                let mut macro_lexer = Lexer::new(macro_source.as_str());

                let mut statements: Vec<StatementContainer<dyn Statement>> = Vec::new();

                loop {
                    match macro_lexer.next() {
                        Some(Ok(Token::Comment)) => {}
                        Some(Ok(Token::Identifier(identifier))) => {
                            // TODO: Assembler error locations will be incorrect here as we aren't passing any sort of context with the sub-lexer
                            statements.push(parse_statement(
                                identifier,
                                &mut macro_lexer,
                                macros,
                                &macro_parsing_context,
                            )?);
                        }
                        Some(Ok(token)) => {
                            return Err(AssemblerError::new(
                                format!("Unexpected {:?}, expected Identifier", token),
                                macro_parsing_context.get_backtrace(macro_lexer.span()),
                            ));
                        }
                        Some(Err(e)) => {
                            return Err(AssemblerError::new(
                                format!("Lexer error: {}", e),
                                macro_parsing_context.get_backtrace(macro_lexer.span()),
                            ));
                        }
                        None => break,
                    };
                }

                Box::new(MacroInvocationStatement::new(statements))
            }
            None => {
                return Err(AssemblerError::new(
                    format!("Unrecognized identifier {}", identifier),
                    parsing_context.get_backtrace(lexer.span()),
                ));
            }
        },
    };

    Ok(StatementContainer::new(
        statement,
        parsing_context.get_backtrace(span_start..(lexer.span().end)),
    ))
}

fn parse_add_statement(lexer: &mut Lexer<Token>, parsing_context: &ParsingContext) -> Result<Add, AssemblerError> {
    let destination_register = next_token_unwrapped!(lexer, parsing_context, Token::Register)?;
    let source_register_zero = next_token_unwrapped!(lexer, parsing_context, Token::Register)?;

    let source_one_value = match next_token!(lexer, parsing_context, Token::Register, Token::NumericLiteral)? {
        Token::Register(source_register_one) => (1 << 5) | ((source_register_one) << 2),
        Token::NumericLiteral(numeric_literal) => match encode_unsigned_integer(numeric_literal, 5) {
            Ok(value) => value,
            Err(e) => return Err(AssemblerError::new(e, parsing_context.get_backtrace(lexer.span()))),
        },
        _ => unreachable!(),
    };

    Ok(Add::new(destination_register, source_register_zero, source_one_value))
}

fn parse_sub_statement(lexer: &mut Lexer<Token>, parsing_context: &ParsingContext) -> Result<Sub, AssemblerError> {
    let destination_register = next_token_unwrapped!(lexer, parsing_context, Token::Register)?;
    let source_register_zero = next_token_unwrapped!(lexer, parsing_context, Token::Register)?;

    let source_one_value = match next_token!(lexer, parsing_context, Token::Register, Token::NumericLiteral)? {
        Token::Register(source_register_one) => (1 << 5) | ((source_register_one) << 2),
        Token::NumericLiteral(numeric_literal) => match encode_unsigned_integer(numeric_literal, 5) {
            Ok(value) => value,
            Err(e) => return Err(AssemblerError::new(e, parsing_context.get_backtrace(lexer.span()))),
        },
        _ => unreachable!(),
    };

    Ok(Sub::new(destination_register, source_register_zero, source_one_value))
}

fn parse_load_effective_address_statement(
    lexer: &mut Lexer<Token>,
    parsing_context: &ParsingContext,
) -> Result<LoadEffectiveAddress, AssemblerError> {
    let destination_register = next_token_unwrapped!(lexer, parsing_context, Token::Register)?;

    match next_token!(lexer, parsing_context, Token::NumericLiteral, Token::Label)? {
        Token::NumericLiteral(numeric_literal) => Ok(LoadEffectiveAddress::from_numeric_literal(
            destination_register,
            numeric_literal,
        )),
        Token::Label(label) => Ok(LoadEffectiveAddress::from_label(destination_register, label)),
        _ => unreachable!(),
    }
}

fn parse_load_statement(lexer: &mut Lexer<Token>, parsing_context: &ParsingContext) -> Result<Load, AssemblerError> {
    let destination_register = next_token_unwrapped!(lexer, parsing_context, Token::Register)?;
    let base_register = next_token_unwrapped!(lexer, parsing_context, Token::Register)?;
    let offset = next_token_unwrapped!(lexer, parsing_context, Token::NumericLiteral)?;

    Ok(Load::new(destination_register, base_register, offset))
}

fn parse_load_immediate_statement(
    lexer: &mut Lexer<Token>,
    parsing_context: &ParsingContext,
) -> Result<LoadImmediate, AssemblerError> {
    let destination_register = next_token_unwrapped!(lexer, parsing_context, Token::Register)?;
    let numeric_literal = next_token_unwrapped!(lexer, parsing_context, Token::NumericLiteral)?;

    let encoded_numeric_literal = match encode_unsigned_integer(numeric_literal, 9) {
        Ok(value) => value,
        Err(e) => return Err(AssemblerError::new(e, parsing_context.get_backtrace(lexer.span()))),
    };

    Ok(LoadImmediate::new(destination_register, encoded_numeric_literal))
}

fn parse_store_statement(lexer: &mut Lexer<Token>, parsing_context: &ParsingContext) -> Result<Store, AssemblerError> {
    let base_register = next_token_unwrapped!(lexer, parsing_context, Token::Register)?;
    let offset = next_token_unwrapped!(lexer, parsing_context, Token::NumericLiteral)?;
    let source_register = next_token_unwrapped!(lexer, parsing_context, Token::Register)?;

    Ok(Store::new(base_register, offset, source_register))
}

fn parse_branch_statement(
    lexer: &mut Lexer<Token>,
    parsing_context: &ParsingContext,
) -> Result<Branch, AssemblerError> {
    let conditions = next_token_unwrapped!(lexer, parsing_context, Token::BranchConditons)?.bits();

    match next_token!(lexer, parsing_context, Token::NumericLiteral, Token::Label)? {
        Token::NumericLiteral(numeric_literal) => Ok(Branch::from_numeric_literal(conditions, numeric_literal)),
        Token::Label(label) => Ok(Branch::from_label(conditions, label)),
        _ => unreachable!(),
    }
}

fn parse_call_statement(lexer: &mut Lexer<Token>, parsing_context: &ParsingContext) -> Result<Call, AssemblerError> {
    match next_token!(
        lexer,
        parsing_context,
        Token::Register,
        Token::NumericLiteral,
        Token::Label
    )? {
        Token::Register(base_register) => {
            let offset = next_token_unwrapped!(lexer, parsing_context, Token::NumericLiteral)?;
            let encoded_offset = match encode_signed_integer(offset, 8) {
                Ok(value) => value,
                Err(e) => return Err(AssemblerError::new(e, parsing_context.get_backtrace(lexer.span()))),
            };

            Ok(Call::from_register_and_offset(base_register, encoded_offset))
        }
        Token::NumericLiteral(offset) => {
            let encoded_offset = match encode_signed_integer(offset, 11) {
                Ok(value) => value,
                Err(e) => return Err(AssemblerError::new(e, parsing_context.get_backtrace(lexer.span()))),
            };

            Ok(Call::from_encoded_offset(encoded_offset))
        }
        Token::Label(label) => Ok(Call::from_label(&label)),
        _ => unreachable!(),
    }
}

fn parse_return_statement<'a>(_: &mut Lexer<'a, Token>, _: &ParsingContext) -> Result<Return, AssemblerError> {
    Ok(Return::new())
}

fn parse_halt_statement<'a>(_: &mut Lexer<'a, Token>, _: &ParsingContext) -> Result<Halt, AssemblerError> {
    Ok(Halt::new())
}

fn parse_sleep_statement(lexer: &mut Lexer<Token>, parsing_context: &ParsingContext) -> Result<Sleep, AssemblerError> {
    let duration = next_token_unwrapped!(lexer, parsing_context, Token::NumericLiteral)?;

    let encoded_duration = match encode_unsigned_integer(duration, 12) {
        Ok(value) => value,
        Err(e) => return Err(AssemblerError::new(e, parsing_context.get_backtrace(lexer.span()))),
    };

    Ok(Sleep::new(encoded_duration))
}

fn parse_word_statement(lexer: &mut Lexer<Token>, parsing_context: &ParsingContext) -> Result<Word, AssemblerError> {
    let numeric_literal = next_token_unwrapped!(lexer, parsing_context, Token::NumericLiteral)?;

    let encoded_value = match encode_unsigned_integer(numeric_literal, 16) {
        Ok(value) => value,
        Err(e) => return Err(AssemblerError::new(e, parsing_context.get_backtrace(lexer.span()))),
    };

    Ok(Word::new(encoded_value))
}

fn parse_ascii_statement(lexer: &mut Lexer<Token>, parsing_context: &ParsingContext) -> Result<Ascii, AssemblerError> {
    let string = next_token_unwrapped!(lexer, parsing_context, Token::String)?;

    Ok(Ascii::new(&string))
}

fn parse_block_statement(lexer: &mut Lexer<Token>, parsing_context: &ParsingContext) -> Result<Block, AssemblerError> {
    let numeric_literal = next_token_unwrapped!(lexer, parsing_context, Token::NumericLiteral)?;

    let size = match encode_unsigned_integer(numeric_literal, 16) {
        Ok(value) => value,
        Err(e) => return Err(AssemblerError::new(e, parsing_context.get_backtrace(lexer.span()))),
    };

    Ok(Block::new(size))
}
