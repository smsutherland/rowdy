use crate::{
    lexer::{
        token::{QualifiedToken as Token, QualifiedTokenType as TokenType},
        TokenIter,
    },
    location::Span,
};

use super::{Parse, ParseError, Result, Spanned};

#[derive(Debug)]
pub struct Symbol {
    pub text: String,
    pub span: Span,
}

impl Parse for Symbol {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        if let Some(Token {
            typ: TokenType::Symbol(text),
            span,
        }) = tokens.next()
        {
            Ok(Self { text, span })
        } else {
            Err(ParseError::UnexpectedToken)
        }
    }
}

impl Spanned for Symbol {
    fn span(&self) -> Span {
        self.span
    }
}
