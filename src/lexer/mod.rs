mod token;
use token::*;

#[derive(PartialEq, Eq)]
enum LexState {
    Start,
    Symbol,
    Integer,
}

impl LexState {
    fn call(&self, c: char, lit_val: String) -> StateResult {
        match self {
            Self::Start => state_start(c, lit_val),
            Self::Symbol => state_symbol(c, lit_val),
            Self::Integer => state_integer(c, lit_val),
        }
    }
}

enum StateResult {
    IncompleteToken(LexState, String),
    CompleteToken(TokenType, bool),
}
use StateResult::*;

pub fn lex_file(filename: &str) -> Result<Vec<Token>, String> {
    let lines = match super::read_lines(&filename) {
        Ok(val) => val,
        Err(_) => return Err(format!("Could not read file {}", &filename)),
    };

    let mut tokens = Vec::new();
    let mut state = LexState::Start;
    let mut token_start = Location {
        file: filename,
        line: 0,
        col: 0,
    };
    let mut current_substr = String::new();

    for (line_num, line) in lines.enumerate() {
        if let Ok(line) = line {
            for (col_num, c) in line.chars().enumerate() {
                let mut continue_loop = true;
                while continue_loop {
                    continue_loop = false;
                    if state == LexState::Start {
                        token_start = Location {
                            file: filename,
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
                                loc: token_start,
                            });
                            continue_loop = go_back;
                        }
                    };
                }
            }
        } else {
            todo!("Error handling");
        }
    }

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

fn state_start(c: char, lit_val: String) -> StateResult {
    match c {
        ';' => CompleteToken(TokenType::End, false),
        '(' => CompleteToken(TokenType::SpecialChar(SpecialChar::LParen), false),
        ')' => CompleteToken(TokenType::SpecialChar(SpecialChar::RParen), false),
        '[' => CompleteToken(TokenType::SpecialChar(SpecialChar::LBracket), false),
        ']' => CompleteToken(TokenType::SpecialChar(SpecialChar::RBracket), false),
        '}' => CompleteToken(TokenType::SpecialChar(SpecialChar::LBrace), false),
        '{' => CompleteToken(TokenType::SpecialChar(SpecialChar::RBrace), false),
        '=' => CompleteToken(TokenType::Operator(Operator::Assign), false),
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
                "int" => TokenType::DataType(DataType::Int),
                _ => TokenType::Symbol(lit_val),
            },
            true,
        ),
    }
}

fn state_integer(c: char, mut lit_val: String) -> StateResult {
    use token::TokenType::*;
    match c {
        digit!() => {
            lit_val.push(c);
            IncompleteToken(LexState::Integer, lit_val)
        }
        _ => CompleteToken(IntLit(lit_val.parse().unwrap()), true),
    }
}
