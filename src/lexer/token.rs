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
    TokenSymbol(String),
    TokenOperator(Operator),
    TokenDataType(DataType),
    TokenSpecialChar(SpecialChar),
    TokenEnd,
}

impl TokenType {
    pub fn from_string(token_str: &str) -> Self {
        use DataType::*;
        use Operator::*;
        use SpecialChar::*;
        use TokenType::*;

        match token_str {
            "+" => TokenOperator(Plus),
            "-" => TokenOperator(Sub),
            "=" => TokenOperator(Assign),
            "==" => TokenOperator(Equal),
            ";" => TokenEnd,

            "int" => TokenDataType(Int),

            "(" => TokenSpecialChar(LParen(None)),
            ")" => TokenSpecialChar(RParen(None)),
            "{" => TokenSpecialChar(LBrace(None)),
            "}" => TokenSpecialChar(RBrace(None)),
            "[" => TokenSpecialChar(LBracket(None)),
            "]" => TokenSpecialChar(RBracket(None)),

            _ => TokenSymbol(String::from(token_str)),
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TokenSymbol(_) => write!(f, "symbol"),
            Self::TokenOperator(_) => write!(f, "operator"),
            Self::TokenDataType(_) => write!(f, "data type"),
            Self::TokenSpecialChar(_) => write!(f, "special char"),
            Self::TokenEnd => write!(f, "end"),
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
