use crate::location::Span;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Token {
    pub typ: TokenType,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Symbol,
    Operator(Operator),
    SpecialChar(SpecialChar),
    IntLit,
    FloatLit,
    End,
    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Symbol => write!(f, "symbol"),
            Self::Operator(_) => write!(f, "operator"),
            Self::SpecialChar(_) => write!(f, "special char"),
            Self::IntLit => write!(f, "int lit"),
            Self::FloatLit => write!(f, "float lit"),
            Self::End => write!(f, "end"),
            Self::Eof => write!(f, "eof"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Plus,
    PlusAssign,
    Increment,
    Sub,
    SubAssign,
    Decrement,
    Assign,
    Equals,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecialChar {
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    If,
    Else,
    While,
    For,
    Return,
}

#[derive(Debug, Clone, PartialEq)]
pub enum QualifiedTokenType {
    Symbol(String),
    Operator(Operator),
    SpecialChar(SpecialChar),
    IntLit(i32),
    FloatLit(f32),
    Keyword(Keyword),
    End,
    Eof,
}

#[derive(Debug, Clone)]
pub struct QualifiedToken {
    pub typ: QualifiedTokenType,
    pub span: Span,
}

pub fn qualify_token(token: Token, code: &str) -> QualifiedToken {
    match token.typ {
        TokenType::Symbol => {
            let symbol = token.span.slice(code);
            match symbol {
                "if" => QualifiedToken {
                    typ: QualifiedTokenType::Keyword(Keyword::If),
                    span: token.span,
                },
                "for" => QualifiedToken {
                    typ: QualifiedTokenType::Keyword(Keyword::For),
                    span: token.span,
                },
                "else" => QualifiedToken {
                    typ: QualifiedTokenType::Keyword(Keyword::Else),
                    span: token.span,
                },
                "while" => QualifiedToken {
                    typ: QualifiedTokenType::Keyword(Keyword::While),
                    span: token.span,
                },
                "return" => QualifiedToken {
                    typ: QualifiedTokenType::Keyword(Keyword::Return),
                    span: token.span,
                },
                _ => QualifiedToken {
                    typ: QualifiedTokenType::Symbol(symbol.to_string()),
                    span: token.span,
                },
            }
        }
        TokenType::Operator(op) => QualifiedToken {
            typ: QualifiedTokenType::Operator(op),
            span: token.span,
        },
        TokenType::SpecialChar(c) => QualifiedToken {
            typ: QualifiedTokenType::SpecialChar(c),
            span: token.span,
        },
        TokenType::IntLit => QualifiedToken {
            typ: QualifiedTokenType::FloatLit(
                token
                    .span
                    .slice(code)
                    .parse()
                    .expect("Failed to parse int lit"),
            ),
            span: token.span,
        },
        TokenType::FloatLit => QualifiedToken {
            typ: QualifiedTokenType::FloatLit(
                token
                    .span
                    .slice(code)
                    .parse()
                    .expect("Failed to parse float lit"),
            ),
            span: token.span,
        },
        TokenType::End => QualifiedToken {
            typ: QualifiedTokenType::End,
            span: token.span,
        },
        TokenType::Eof => QualifiedToken {
            typ: QualifiedTokenType::Eof,
            span: token.span,
        },
    }
}
