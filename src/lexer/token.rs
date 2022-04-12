use std::fmt;

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

#[derive(Debug, Clone)]
pub enum TokenType {
    Symbol(String),
    Operator(Operator),
    DataType(DataType),
    SpecialChar(SpecialChar),
    IntLit(i32), // TODO: Do we need to increase this to i64?
    End,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Symbol(_) => write!(f, "symbol"),
            Self::Operator(_) => write!(f, "operator"),
            Self::DataType(_) => write!(f, "data type"),
            Self::SpecialChar(_) => write!(f, "special char"),
            Self::End => write!(f, "end"),
            Self::IntLit(_) => write!(f, "int lit"),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum DataType {
    Int,
}

#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    Sub,
    Assign,
    Equal,
}

#[derive(Debug, Clone)]
pub enum SpecialChar {
    LParen(Option<usize>),
    RParen(Option<usize>),
    LBrace(Option<usize>),
    RBrace(Option<usize>),
    LBracket(Option<usize>),
    RBracket(Option<usize>),
}
