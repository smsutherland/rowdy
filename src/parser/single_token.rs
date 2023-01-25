use crate::{
    lexer::{
        token::{QualifiedToken as Token, QualifiedTokenType as TokenType, SpecialChar},
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

macro_rules! Token {
    [,] => {
        $self::Comma
    };
}

macro_rules! special_char_node {
    ($name:ident) => {
        #[derive(Debug)]
        struct $name {
            span: Span,
        }

        impl Parse for $name {
            fn parse(tokens: &mut TokenIter) -> Result<Self> {
                if let Some(Token {
                    typ: TokenType::SpecialChar(SpecialChar::$name),
                    span,
                }) = tokens.next()
                {
                    Ok(Self { span })
                } else {
                    Err(ParseError::UnexpectedToken)
                }
            }
        }

        impl Spanned for $name {
            fn span(&self) -> Span {
                self.span
            }
        }
    };
}

special_char_node!(LParen);
special_char_node!(RParen);
special_char_node!(LBrace);
special_char_node!(RBrace);
special_char_node!(LBracket);
special_char_node!(RBracket);
special_char_node!(Comma);
