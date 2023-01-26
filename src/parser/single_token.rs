use crate::{
    lexer::{
        token::{
            Keyword, Operator, QualifiedToken as Token, QualifiedTokenType as TokenType,
            SpecialChar,
        },
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
        match tokens.next() {
            Some(Token {
                typ: TokenType::Symbol(text),
                span,
            }) => Ok(Self { text, span }),
            Some(other) => Err(ParseError::UnexpectedToken {
                expected: "Symbol",
                got: other.typ,
            }),
            None => Err(ParseError::OutOfTokens),
        }
    }
}

impl Spanned for Symbol {
    fn span(&self) -> Span {
        self.span
    }
}

#[macro_export]
macro_rules! Token {
    [,] => {$crate::parser::single_token::Comma};
    [;] => {$crate::parser::single_token::End};
    [+] => {$crate::parser::single_token::Plus};
    [+=] => {$crate::parser::single_token::PlusAssign};
    [++] => {$crate::parser::single_token::Increment};
    [-] => {$crate::parser::single_token::Sub};
    [-=] => {$crate::parser::single_token::SubAssign};
    [--] => {$crate::parser::single_token::Decrement};
    [=] => {$crate::parser::single_token::Assign};
    [==] => {$crate::parser::single_token::Equals};
}

macro_rules! make_node {
    ($kind:ident :: $name:ident) => {
        #[derive(Debug)]
        pub struct $name {
            span: Span,
        }

        impl Parse for $name {
            fn parse(tokens: &mut TokenIter) -> Result<Self> {
                match tokens.next() {
                    Some(Token {
                        typ: TokenType::$kind($kind::$name),
                        span,
                    }) => Ok(Self { span }),
                    Some(other) => Err(ParseError::UnexpectedToken {
                        expected: stringify!($name),
                        got: other.typ,
                    }),
                    None => Err(ParseError::OutOfTokens),
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

macro_rules! special_char_node {
    ($name:ident) => {
        make_node! {SpecialChar::$name}
    };
}

special_char_node! {LParen}
special_char_node! {RParen}
special_char_node! {LBrace}
special_char_node! {RBrace}
special_char_node! {LBracket}
special_char_node! {RBracket}
special_char_node! {Comma}

#[derive(Debug)]
pub struct End {
    span: Span,
}

impl Parse for End {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(Token {
                typ: TokenType::End,
                span,
            }) => Ok(Self { span }),
            Some(other) => Err(ParseError::UnexpectedToken {
                expected: "End",
                got: other.typ,
            }),
            None => Err(ParseError::OutOfTokens),
        }
    }
}

impl Spanned for End {
    fn span(&self) -> Span {
        self.span
    }
}

macro_rules! operator_node {
    ($name:ident) => {
        make_node! {Operator::$name}
    };
}

operator_node! {Plus}
operator_node! {PlusAssign}
operator_node! {Increment}
operator_node! {Sub}
operator_node! {SubAssign}
operator_node! {Decrement}
operator_node! {Assign}
operator_node! {Equals}

#[derive(Debug)]
pub struct IntLit {
    span: Span,
    value: i32,
}

impl Parse for IntLit {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(Token {
                typ: TokenType::IntLit(value),
                span,
            }) => Ok(Self { span, value }),
            Some(other) => Err(ParseError::UnexpectedToken {
                expected: "IntLit",
                got: other.typ,
            }),
            None => Err(ParseError::OutOfTokens),
        }
    }
}

impl Spanned for IntLit {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug)]
pub struct FloatLit {
    span: Span,
    value: f32,
}

impl Parse for FloatLit {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next() {
            Some(Token {
                typ: TokenType::FloatLit(value),
                span,
            }) => Ok(Self { span, value }),
            Some(other) => Err(ParseError::UnexpectedToken {
                expected: "IntLit",
                got: other.typ,
            }),
            None => Err(ParseError::OutOfTokens),
        }
    }
}

impl Spanned for FloatLit {
    fn span(&self) -> Span {
        self.span
    }
}

macro_rules! keyword_node {
    ($name:ident) => {
        make_node! {Keyword::$name}
    };
}

keyword_node! {If}
keyword_node! {Else}
keyword_node! {While}
keyword_node! {For}
keyword_node! {Return}
