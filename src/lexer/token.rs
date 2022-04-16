use std::fmt;

use crate::types::Type;

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub typ: TokenType,
    pub loc: Location<'a>,
}

#[derive(Clone, Copy)]
pub struct Location<'a> {
    pub file: &'a str,
    pub line: usize,
    pub col: usize,
}

impl<'a> Location<'a> {
    /// Definition of the location of the end of a file.
    /// All EOF tokens should have their locations come from this function.
    pub fn end(file: &'a str) -> Self {
        Self {
            file,
            line: 0,
            col: 0,
        }
    }
}

impl fmt::Display for Location<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.col)
    }
}

impl fmt::Debug for Location<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Symbol(String),
    Operator(Operator),
    DataType(Type),
    SpecialChar(SpecialChar),
    IntLit(i32), // TODO: Do we need to increase this to i64? Have multiple IntLit types?
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
            Self::End => write!(f, "end"),
            Self::Eof => write!(f, "eof"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Sub,
    Assign,
    Equal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecialChar {
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
}
