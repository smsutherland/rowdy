use crate::{lexer::token::QualifiedTokenType as TokenType, lexer::TokenIter, location::Span};
use node::Function;

mod node;
mod single_token;

#[derive(Debug)]
pub struct Program {
    functions: Vec<Function>,
}

pub type Ast = Program;

pub fn parse_tokens(tokens: TokenIter) -> Ast {
    let mut tokens = tokens;
    let mut functions = Vec::new();
    while !tokens.is_empty() {
        functions.push(parse(&mut tokens).unwrap());
    }

    Program { functions }
}

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken {
        expected: &'static str,
        got: TokenType,
    },
    OutOfTokens,
}

pub trait Parse: Sized {
    fn parse(tokens: &mut TokenIter) -> Result<Self>;

    fn try_parse(tokens: &mut TokenIter) -> Result<Self> {
        let mut tokens_clone = tokens.clone();
        let result = Self::parse(&mut tokens_clone)?;
        *tokens = tokens_clone;
        Ok(result)
    }
}

#[inline]
pub fn parse<T: Parse>(tokens: &mut TokenIter) -> Result<T> {
    T::parse(tokens)
}

#[inline]
pub fn try_parse<T: Parse>(tokens: &mut TokenIter) -> Result<T> {
    T::try_parse(tokens)
}

pub trait Spanned {
    fn span(&self) -> Span;
}
