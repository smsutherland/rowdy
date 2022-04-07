mod token;
use token::*;

#[derive(PartialEq, Eq)]
enum LexState {
    Start,
    Symbol,
}

impl LexState {
    fn call(&self, c: char, lit_val: String) -> StateResult {
        match self {
            Self::Start => state_start(c, lit_val),
            Self::Symbol => state_symbol(c, lit_val),
        }
    }
}

struct StateResult {
    next_state: LexState,
    next_token: Option<TokenType>,
    next_substr: String,
    go_back: bool,
}

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
                loop {
                    if state == LexState::Start {
                        token_start = Location {
                            file: filename,
                            line: line_num + 1, // line numbers are 1-indexed
                            col: col_num + 1,   // col numbers are 1-indexed
                        };
                    }
                    let call_result = state.call(c, current_substr);
                    current_substr = call_result.next_substr;
                    state = call_result.next_state;
                    if let Some(next_token) = call_result.next_token {
                        tokens.push(Token {
                            typ: next_token,
                            loc: token_start,
                        });
                    }
                    if !call_result.go_back {
                        break;
                    }
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

macro_rules! digit_nonzero {
    () => {
        '1'..='9'
    };
}

macro_rules! digit {
    () => {
        '0'..='9'
    };
}

fn state_start(c: char, lit_val: String) -> StateResult {
    use token::TokenType::*;
    match c {
        ';' => StateResult {
            next_state: LexState::Start,
            next_token: Some(TokenEnd),
            next_substr: String::new(),
            go_back: false,
        },
        '(' => StateResult {
            next_state: LexState::Start,
            next_token: Some(TokenSpecialChar(SpecialChar::LParen(None))),
            next_substr: String::new(),
            go_back: false,
        },
        ')' => StateResult {
            next_state: LexState::Start,
            next_token: Some(TokenSpecialChar(SpecialChar::RParen(None))),
            next_substr: String::new(),
            go_back: false,
        },
        '[' => StateResult {
            next_state: LexState::Start,
            next_token: Some(TokenSpecialChar(SpecialChar::LBracket(None))),
            next_substr: String::new(),
            go_back: false,
        },
        ']' => StateResult {
            next_state: LexState::Start,
            next_token: Some(TokenSpecialChar(SpecialChar::RBracket(None))),
            next_substr: String::new(),
            go_back: false,
        },
        '}' => StateResult {
            next_state: LexState::Start,
            next_token: Some(TokenSpecialChar(SpecialChar::LBrace(None))),
            next_substr: String::new(),
            go_back: false,
        },
        '{' => StateResult {
            next_state: LexState::Start,
            next_token: Some(TokenSpecialChar(SpecialChar::RBrace(None))),
            next_substr: String::new(),
            go_back: false,
        },
        '=' => StateResult {
            next_state: LexState::Start,
            next_token: Some(TokenOperator(Operator::Assign)),
            next_substr: String::new(),
            go_back: false,
        },
        symbol_start!() => StateResult {
            next_state: LexState::Symbol,
            next_token: None,
            next_substr: String::from(c),
            go_back: false,
        },
        whitespace!() => StateResult {
            next_state: LexState::Start,
            next_token: None,
            next_substr: String::new(),
            go_back: false,
        },
        _ => {
            panic!("unrecognized character '{c}'");
        }
    }
}

macro_rules! symbol_mid {
    () => {
        symbol_start!() | '0'..='9'
    };
}

fn state_symbol<'a>(c: char, mut lit_val: String) -> StateResult {
    use token::TokenType::*;
    match c {
        symbol_mid!() => {
            lit_val.push(c);
            StateResult {
                next_state: LexState::Symbol,
                next_token: None,
                next_substr: lit_val,
                go_back: false,
            }
        }
        _ => StateResult {
            next_state: LexState::Start,
            next_token: Some(TokenSymbol(lit_val)),
            next_substr: String::new(),
            go_back: true,
        },
    }
}
