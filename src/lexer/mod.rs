mod cursor;
pub mod token;
pub use crate::location::{Location, Span};
use crate::Compiler;
use cursor::Cursor;
pub use token::qualify_token;
use token::*;

#[derive(Debug)]
pub struct TokenIter<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        next_token(&mut self.cursor)
    }
}

pub fn tokenize(compiler: &Compiler) -> TokenIter {
    tokenize_str(&compiler.code)
}

pub fn tokenize_str(input: &str) -> TokenIter {
    let cursor = Cursor::new(input);
    TokenIter { cursor }
}

fn next_token(cursor: &mut Cursor) -> Option<Token> {
    loop {
        let (next, start_loc) = match cursor.next() {
            Some(result) => result,
            None => return None,
        };
        break Some(match next {
            ';' => Token {
                typ: TokenType::End,
                span: Span::from_loc(start_loc),
            },
            '(' => Token {
                typ: TokenType::SpecialChar(SpecialChar::LParen),
                span: Span::from_loc(start_loc),
            },
            ')' => Token {
                typ: TokenType::SpecialChar(SpecialChar::RParen),
                span: Span::from_loc(start_loc),
            },
            '[' => Token {
                typ: TokenType::SpecialChar(SpecialChar::LBracket),
                span: Span::from_loc(start_loc),
            },
            ']' => Token {
                typ: TokenType::SpecialChar(SpecialChar::RBracket),
                span: Span::from_loc(start_loc),
            },
            '{' => Token {
                typ: TokenType::SpecialChar(SpecialChar::LBrace),
                span: Span::from_loc(start_loc),
            },
            '}' => Token {
                typ: TokenType::SpecialChar(SpecialChar::RBrace),
                span: Span::from_loc(start_loc),
            },
            ',' => Token {
                typ: TokenType::SpecialChar(SpecialChar::Comma),
                span: Span::from_loc(start_loc),
            },
            '=' => match cursor.peek(0) {
                Some(('=', end_loc)) => {
                    cursor.consume(0);
                    Token {
                        typ: TokenType::Operator(Operator::Equals),
                        span: Span::from_start_end(start_loc, end_loc),
                    }
                }
                _ => Token {
                    typ: TokenType::Operator(Operator::Assign),
                    span: Span::from_loc(start_loc),
                },
            },
            '+' => match cursor.peek(0) {
                Some(('+', end_loc)) => {
                    cursor.consume(0);
                    Token {
                        typ: TokenType::Operator(Operator::Increment),
                        span: Span::from_start_end(start_loc, end_loc),
                    }
                }
                Some(('=', end_loc)) => {
                    cursor.consume(0);
                    Token {
                        typ: TokenType::Operator(Operator::PlusAssign),
                        span: Span::from_start_end(start_loc, end_loc),
                    }
                }
                _ => Token {
                    typ: TokenType::Operator(Operator::Plus),
                    span: Span::from_loc(start_loc),
                },
            },
            '-' => match cursor.peek(0) {
                Some(('-', end_loc)) => {
                    cursor.consume(0);
                    Token {
                        typ: TokenType::Operator(Operator::Decrement),
                        span: Span::from_start_end(start_loc, end_loc),
                    }
                }
                Some(('=', end_loc)) => {
                    cursor.consume(0);
                    Token {
                        typ: TokenType::Operator(Operator::SubAssign),
                        span: Span::from_start_end(start_loc, end_loc),
                    }
                }
                _ => Token {
                    typ: TokenType::Operator(Operator::Sub),
                    span: Span::from_loc(start_loc),
                },
            },
            c if c.is_ascii_digit() => cursor.number(start_loc),
            c if is_symbol_start(&c) => cursor.symbol(start_loc),
            c if c.is_whitespace() => continue,

            c => todo!("'{}'", c),
        });
    }
}

#[inline]
fn is_symbol_start(c: &char) -> bool {
    matches!(c, 'a'..='z'|'A'..='Z'|'_')
}

#[inline]
fn is_symbol_middle(c: &char) -> bool {
    matches!(c,  'a'..='z'|'A'..='Z'|'_'|'0'..='9')
}

impl<'a> Cursor<'a> {
    fn number(&mut self, start_loc: Location) -> Token {
        let end_loc = self.eat_while(char::is_ascii_digit);
        if let Some('.') = self.peek_char(0) {
            self.consume(0);
            let end_loc = self.eat_while(char::is_ascii_digit);
            Token {
                typ: TokenType::FloatLit,
                span: Span::from_start_end(start_loc, end_loc.unwrap_or(start_loc)),
            }
        } else {
            Token {
                typ: TokenType::IntLit,
                span: Span::from_start_end(start_loc, end_loc.unwrap_or(start_loc)),
            }
        }
    }

    fn symbol(&mut self, start_loc: Location) -> Token {
        let end_loc = self.eat_while(is_symbol_middle);
        Token {
            typ: TokenType::Symbol,
            span: Span::from_start_end(start_loc, end_loc.unwrap_or(start_loc)),
        }
    }
}
