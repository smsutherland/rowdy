mod cursor;
mod location;
pub mod token;
use cursor::Cursor;
pub use location::{Location, Span};
use token::*;

pub fn tokenize<'a>(input: &'a str) -> impl Iterator<Item = Token> + 'a {
    let mut cursor = Cursor::<'a>::new(input);
    std::iter::from_fn(move || next_token(&mut cursor))
}

fn next_token<'a>(cursor: &mut Cursor<'a>) -> Option<Token> {
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
                span: Span::from_start_end(start_loc, end_loc),
            }
        } else {
            Token {
                typ: TokenType::IntLit,
                span: Span::from_start_end(start_loc, end_loc),
            }
        }
    }

    fn symbol(&mut self, start_loc: Location) -> Token {
        let end_loc = self.eat_while(is_symbol_middle);
        Token {
            typ: TokenType::Symbol,
            span: Span::from_start_end(start_loc, end_loc),
        }
    }
}
