use std::fmt;

use crate::location::Span;

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
    Keyword(Keyword),
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
            Self::Keyword(_) => write!(f, "keyword"),
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
