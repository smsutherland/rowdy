use std::fmt;

use super::location::{Location, Span};
use crate::types::Type;

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub typ: TokenType,
    pub span: Span<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Symbol(String),
    Operator(Operator),
    DataType(Type),
    SpecialChar(SpecialChar),
    IntLit(i32), // TODO: Do we need to increase this to i64? Have multiple IntLit types?
    FloatLit(f32),
    Keyword(Keyword),
    End,
    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Symbol(_) => write!(f, "symbol"),
            Self::Operator(_) => write!(f, "operator"),
            Self::DataType(_) => write!(f, "data type"),
            Self::SpecialChar(_) => write!(f, "special char"),
            Self::IntLit(_) => write!(f, "int lit"),
            Self::FloatLit(_) => write!(f, "float lit"),
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
