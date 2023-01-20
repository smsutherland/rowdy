pub mod token;
use crate::types::Type;
use token::*;

#[derive(PartialEq, Eq)]
enum LexState {
    Start,
    Symbol,
    Integer,
    Float,
    Equals,
    Plus,
    Minus,
}

impl LexState {
    fn call(&self, c: char, lit_val: String) -> StateResult {
        match self {
            Self::Start => state_start(c),
            Self::Symbol => state_symbol(c, lit_val),
            Self::Integer => state_integer(c, lit_val),
            Self::Float => state_float(c, lit_val),
            Self::Equals => state_equals(c),
            Self::Plus => state_plus(c),
            Self::Minus => state_minus(c),
        }
    }
}

enum StateResult {
    IncompleteToken(LexState, String),
    CompleteToken(TokenType, bool),
}
use StateResult::*;

pub fn lex_file(filename: &str) -> Result<Vec<Token>, String> {
    let lines = match super::read_lines(filename) {
        Ok(val) => val,
        Err(error) => return Err(format!("Could not read file {} - {}", filename, error)),
    };

    let mut tokens = Vec::new();
    let mut state = LexState::Start;
    let mut token_start = Location { line: 0, col: 0 };
    let mut current_substr = String::new();

    for (line_num, line) in lines.enumerate() {
        if let Ok(line) = line {
            for (col_num, c) in line.chars().enumerate() {
                let mut continue_loop = true;
                while continue_loop {
                    continue_loop = false;
                    if state == LexState::Start {
                        token_start = Location {
                            line: line_num + 1, // line numbers are 1-indexed
                            col: col_num + 1,   // col numbers are 1-indexed
                        };
                    }
                    match state.call(c, current_substr) {
                        IncompleteToken(next_state, next_substr) => {
                            state = next_state;
                            current_substr = next_substr;
                        }
                        CompleteToken(next_token, go_back) => {
                            state = LexState::Start;
                            current_substr = String::new();
                            tokens.push(Token {
                                typ: next_token,
                                span: todo!(),
                            });
                            continue_loop = go_back;
                        }
                    }
                }
            }
        } else {
            todo!("Error handling");
        }
    }

    tokens.push(Token {
        span: todo!(),
        typ: TokenType::Eof,
    });
    Ok(tokens)
}

macro_rules! symbol_start {
    () => {
        'a'..='z'|'A'..='Z'|'_'
    };
}

macro_rules! whitespace {
    () => {
        ' ' | '\t'
    };
}

macro_rules! digit {
    () => {
        '0'..='9'
    };
}

fn state_start(c: char) -> StateResult {
    match c {
        ';' => CompleteToken(TokenType::End, false),
        '(' => CompleteToken(TokenType::SpecialChar(SpecialChar::LParen), false),
        ')' => CompleteToken(TokenType::SpecialChar(SpecialChar::RParen), false),
        '[' => CompleteToken(TokenType::SpecialChar(SpecialChar::LBracket), false),
        ']' => CompleteToken(TokenType::SpecialChar(SpecialChar::RBracket), false),
        '{' => CompleteToken(TokenType::SpecialChar(SpecialChar::LBrace), false),
        '}' => CompleteToken(TokenType::SpecialChar(SpecialChar::RBrace), false),
        ',' => CompleteToken(TokenType::SpecialChar(SpecialChar::Comma), false),
        '=' => IncompleteToken(LexState::Equals, String::from(c)),
        '+' => IncompleteToken(LexState::Plus, String::from(c)),
        '-' => IncompleteToken(LexState::Minus, String::from(c)),
        symbol_start!() => IncompleteToken(LexState::Symbol, String::from(c)),
        whitespace!() => IncompleteToken(LexState::Start, String::new()),
        digit!() => IncompleteToken(LexState::Integer, String::from(c)),
        _ => {
            panic!("unrecognized character '{c}'");
        }
    }
}

macro_rules! symbol_mid {
    () => {
        symbol_start!() | digit!()
    };
}

fn state_symbol(c: char, mut lit_val: String) -> StateResult {
    match c {
        symbol_mid!() => {
            lit_val.push(c);
            IncompleteToken(LexState::Symbol, lit_val)
        }
        _ => CompleteToken(
            match lit_val.as_str() {
                "int" => TokenType::DataType(Type::Int),
                "bool" => TokenType::DataType(Type::Bool),
                "float" => TokenType::DataType(Type::Float),
                "if" => TokenType::Keyword(Keyword::If),
                "else" => TokenType::Keyword(Keyword::Else),
                "while" => TokenType::Keyword(Keyword::While),
                "for" => TokenType::Keyword(Keyword::For),
                "return" => TokenType::Keyword(Keyword::Return),
                _ => TokenType::Symbol(lit_val),
            },
            true,
        ),
    }
}

fn state_integer(c: char, mut lit_val: String) -> StateResult {
    match c {
        digit!() => {
            lit_val.push(c);
            IncompleteToken(LexState::Integer, lit_val)
        }
        '.' => {
            lit_val.push(c);
            IncompleteToken(LexState::Float, lit_val)
        }
        _ => CompleteToken(TokenType::IntLit(lit_val.parse().unwrap()), true),
    }
}

fn state_float(c: char, mut lit_val: String) -> StateResult {
    match c {
        digit!() => {
            lit_val.push(c);
            IncompleteToken(LexState::Float, lit_val)
        }
        _ => CompleteToken(TokenType::FloatLit(lit_val.parse().unwrap()), true),
    }
}

fn state_equals(c: char) -> StateResult {
    match c {
        '=' => CompleteToken(TokenType::Operator(Operator::Equals), false),
        _ => CompleteToken(TokenType::Operator(Operator::Assign), true),
    }
}

fn state_plus(c: char) -> StateResult {
    match c {
        '+' => CompleteToken(TokenType::Operator(Operator::Increment), false),
        '=' => CompleteToken(TokenType::Operator(Operator::PlusAssign), false),
        _ => CompleteToken(TokenType::Operator(Operator::Plus), true),
    }
}

fn state_minus(c: char) -> StateResult {
    match c {
        '-' => CompleteToken(TokenType::Operator(Operator::Decrement), false),
        '=' => CompleteToken(TokenType::Operator(Operator::SubAssign), false),
        _ => CompleteToken(TokenType::Operator(Operator::Sub), true),
    }
}
