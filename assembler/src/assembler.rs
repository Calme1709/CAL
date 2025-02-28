use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter, Result as FormatResult},
    fs,
    ops::{AddAssign, Range},
    path::{absolute, Path, PathBuf},
};

use crate::{
    statements::{
        Add, Ascii, Block, Branch, Call, Halt, Load, LoadEffectiveAddress, LoadImmediate, Return, Sleep, Statement,
        StatementContainer, Store, Sub, Word,
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

    /**
     * A parsing context is recursive if the same source location appears more than once
     */
    pub fn is_recursive(&self) -> bool {
        for i in 0..self.backtrace.len() {
            for l in (i + 1)..self.backtrace.len() {
                if *self.backtrace.get(i).unwrap() == *self.backtrace.get(l).unwrap() {
                    return true;
                }
            }
        }

        return false;
    }
}

#[derive(Clone)]
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

pub fn assemble(file: String) -> Result<Vec<u16>, AssemblerError> {
    let mut label_map: HashMap<String, u16> = HashMap::new();
    let mut label_address = 0;
    let mut macros: HashMap<String, Macro> = HashMap::new();
    let mut included_files: HashSet<String> = HashSet::new();

    let parsing_context = ParsingContext::new(file.clone(), 0, Vec::new());

    let statements = parse_file(
        file.clone(),
        &mut label_map,
        &mut label_address,
        &mut macros,
        &mut included_files,
        parsing_context,
    )?;

    let mut out = Vec::new();
    let mut statement_address = 0;

    for statement in statements {
        out.append(statement.assemble(statement_address, &label_map)?.as_mut());
        statement_address += statement.width();
    }

    return Ok(out);
}

fn parse_file(
    file: String,
    label_map: &mut HashMap<String, u16>,
    label_address: &mut u16,
    macros: &mut HashMap<String, Macro>,
    included_files: &mut HashSet<String>,
    parsing_context: ParsingContext,
) -> Result<Vec<StatementContainer<dyn Statement>>, AssemblerError> {
    included_files.insert(file.clone());

    let source = match fs::read_to_string(std::path::Path::new(&file)) {
        Ok(source) => source,
        Err(e) => {
            return Err(AssemblerError::new(
                format!("Unable to read file {}: {}", file, e.to_string()),
                parsing_context.backtrace,
            ))
        }
    };

    let mut lexer = Lexer::new(source.as_str());
    let mut statements = Vec::new();

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

                label_map.insert(label_name, *label_address);
            }
            Token::Identifier(identifier) => {
                let mut parsed_statements = parse_statement(
                    identifier,
                    &mut lexer,
                    label_map,
                    label_address,
                    macros,
                    included_files,
                    &parsing_context,
                )?;

                statements.append(&mut parsed_statements);
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

    return Ok(statements);
}

fn parse_statement(
    identifier: String,
    lexer: &mut Lexer<Token>,
    label_map: &mut HashMap<String, u16>,
    label_address: &mut u16,
    macros: &mut HashMap<String, Macro>,
    included_files: &mut HashSet<String>,
    parsing_context: &ParsingContext,
) -> Result<Vec<StatementContainer<dyn Statement>>, AssemblerError> {
    let span_start = lexer.span().start;

    // Parse any non-recursive statements (i.e. not macros or file includes)
    let non_recursive_statement: Option<Box<dyn Statement>> = match identifier.as_ref() {
        // Instructions
        "ADD" => Some(Box::new(parse_add_statement(lexer, parsing_context)?)),
        "SUB" => Some(Box::new(parse_sub_statement(lexer, parsing_context)?)),
        "LEA" => Some(Box::new(parse_load_effective_address_statement(
            lexer,
            parsing_context,
        )?)),
        "LD" => Some(Box::new(parse_load_statement(lexer, parsing_context)?)),
        "LDI" => Some(Box::new(parse_load_immediate_statement(lexer, parsing_context)?)),
        "ST" => Some(Box::new(parse_store_statement(lexer, parsing_context)?)),
        "BR" => Some(Box::new(parse_branch_statement(lexer, parsing_context)?)),
        "CALL" => Some(Box::new(parse_call_statement(lexer, parsing_context)?)),
        "RET" => Some(Box::new(parse_return_statement(lexer, parsing_context)?)),
        "HLT" => Some(Box::new(parse_halt_statement(lexer, parsing_context)?)),
        "SLP" => Some(Box::new(parse_sleep_statement(lexer, parsing_context)?)),

        // Directives
        "WORD" => Some(Box::new(parse_word_statement(lexer, parsing_context)?)),
        "ASCII" => Some(Box::new(parse_ascii_statement(lexer, parsing_context)?)),
        "BLK" => Some(Box::new(parse_block_statement(lexer, parsing_context)?)),

        _ => None,
    };

    let statements = match non_recursive_statement {
        Some(statement) => {
            // If the statement was non-recursive it will have already been parsed - increment the label address and
            // wrap the statement to be a valid return type.
            label_address.add_assign(statement.width());

            vec![StatementContainer::new(
                statement,
                parsing_context.get_backtrace(span_start..(lexer.span().end)),
            )]
        }
        None => match identifier.as_ref() {
            // If the label is recrusive - parse it
            "INCLUDE" => parse_include_statement(
                lexer,
                label_map,
                label_address,
                macros,
                included_files,
                parsing_context,
                false,
            )?,
            "INCLUDE_ONCE" => parse_include_statement(
                lexer,
                label_map,
                label_address,
                macros,
                included_files,
                parsing_context,
                true,
            )?,
            _ if macros.get(&identifier).is_some() => parse_macro_invocation(
                macros.get(&identifier).unwrap().clone(),
                lexer,
                label_map,
                label_address,
                macros,
                included_files,
                parsing_context,
            )?,
            _ => {
                return Err(AssemblerError::new(
                    format!("Unrecognized identifier {}", identifier),
                    parsing_context.get_backtrace(lexer.span()),
                ));
            }
        },
    };

    Ok(statements)
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

fn parse_include_statement(
    lexer: &mut Lexer<Token>,
    label_map: &mut HashMap<String, u16>,
    label_address: &mut u16,
    macros: &mut HashMap<String, Macro>,
    included_files: &mut HashSet<String>,
    parsing_context: &ParsingContext,
    include_once: bool,
) -> Result<Vec<StatementContainer<dyn Statement>>, AssemblerError> {
    let include_statement_start = lexer.span().start;

    let relative_path_string = next_token_unwrapped!(lexer, parsing_context, Token::String)?;

    let mut absolute_path_buf = PathBuf::from(parsing_context.file.clone());
    absolute_path_buf.pop();
    absolute_path_buf.push(Path::new(&relative_path_string));

    let file_path = absolute(absolute_path_buf.as_path())
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    if include_once && included_files.contains(&file_path) {
        return Ok(Vec::new());
    }

    let included_file_parsing_context = ParsingContext::new(
        file_path.clone(),
        0,
        parsing_context.get_backtrace(include_statement_start..lexer.span().end),
    );

    if included_file_parsing_context.is_recursive() {
        return Err(AssemblerError::new(
            "Detected recursive file include".to_string(),
            included_file_parsing_context.backtrace,
        ));
    }

    parse_file(
        file_path.clone(),
        label_map,
        label_address,
        macros,
        included_files,
        included_file_parsing_context,
    )
}

fn parse_macro_invocation(
    r#macro: Macro,
    lexer: &mut Lexer<Token>,
    label_map: &mut HashMap<String, u16>,
    label_address: &mut u16,
    macros: &mut HashMap<String, Macro>,
    included_files: &mut HashSet<String>,
    parsing_context: &ParsingContext,
) -> Result<Vec<StatementContainer<dyn Statement>>, AssemblerError> {
    let macro_parsing_context = ParsingContext::new(
        r#macro.definition_file.clone(),
        r#macro.definition_offset,
        parsing_context.get_backtrace(lexer.span()),
    );

    if macro_parsing_context.is_recursive() {
        return Err(AssemblerError::new(
            "Detected recurisve invocation of macro".to_string(),
            macro_parsing_context.backtrace,
        ));
    }

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
                let mut parsed_statements = parse_statement(
                    identifier,
                    &mut macro_lexer,
                    label_map,
                    label_address,
                    macros,
                    included_files,
                    &macro_parsing_context,
                )?;

                statements.append(&mut parsed_statements);
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

    Ok(statements)
}
