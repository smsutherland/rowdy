use std::fmt;

use crate::types::Type;

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub typ: TokenType,
    pub span: Span<'a>,
}

#[derive(Debug, Clone)]
pub struct Span<'a> {
    pub file: Option<&'a str>,
    pub start: Location,
    pub end: Location,
}

impl<'a> Span<'a> {
    pub fn from_loc(loc: Location) -> Self {
        Self {
            file: None,
            start: loc,
            end: loc,
        }
    }

    pub fn from_file_loc(loc: FileLocation<'a>) -> Self {
        Self {
            file: loc.file,
            start: loc.loc,
            end: loc.loc,
        }
    }

    pub fn from_start_end_file(start: FileLocation<'a>, end: FileLocation<'a>) -> Self {
        if start.file != end.file {
            panic!("Cannot create a span across multiple files.");
        }
        Self {
            file: start.file,
            start: start.loc,
            end: end.loc,
        }
    }

    pub fn from_start_end(start: Location<'a>, end: Location<'a>) -> Self {
        Self {
            file: None,
            start: start.loc,
            end: end.loc,
        }
    }
}

#[derive(Clone, Copy)]
pub struct FileLocation<'a> {
    pub file: Option<&'a str>,
    pub loc: Location,
}

#[derive(Clone, Copy)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for FileLocation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            match self.file {
                Some(file) => file,
                None => "anon",
            },
            self.loc.line,
            self.loc.col
        )
    }
}

impl fmt::Debug for FileLocation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
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
