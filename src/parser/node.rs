//! Tokens similar to that of `lexer::token` but aranged into separate types rather than an enum.

use crate::{
    lexer::{
        token::{QualifiedToken as Token, QualifiedTokenType as TokenType},
        TokenIter,
    },
    location::Span,
};

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug)]
enum ParseError {
    UnexpectedToken,
}

pub trait Parse: Sized {
    fn parse(tokens: &mut TokenIter) -> Result<Self>;
}

pub trait Spanned {
    fn span(&self) -> Span;
}

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

#[derive(Debug)]
pub struct Function {
    span: Span,
    return_type: Symbol,
    name: Symbol,
    parameters: Vec<Declaration>,
    expr: BracedExpression,
}

impl Parse for Function {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        let return_type = if let Some(Token {
            typ: TokenType::Symbol(s),
            span,
        }) = tokens.next()
        {
            Symbol { text: s, span }
        } else {
            todo!("Error handling")
        };
        let name = if let Some(Token {
            typ: TokenType::Symbol(s),
            span,
        }) = tokens.next()
        {
            Symbol { text: s, span }
        } else {
            todo!("Error handling")
        };
        match_type!(tokens, TokenType::SpecialChar(token::SpecialChar::LParen));
        let mut parameters = Vec::new();
        while !match_type_peek!(
            tokens,
            QualifiedTokenType::SpecialChar(token::SpecialChar::RParen)
        ) {
            parameters.push(declaration(tokens));
        }
        match_type!(
            tokens,
            QualifiedTokenType::SpecialChar(token::SpecialChar::RParen)
        );

        let expr = braced_expression(tokens);

        Function {
            span: start_span,
            return_type,
            name,
            parameters,
            expr,
        }
    }
}

impl Spanned for Function {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug)]
pub struct Declaration {
    typ: String,
    name: String,
}

#[derive(Debug)]
pub struct BracedExpression {
    statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Declaration(Declaration, Option<Expression>),
    Assignment(String, Expression),
    FunctionCall(String, Vec<Expression>),
}

#[derive(Debug)]
pub enum Expression {
    Braced(BracedExpression),
    IntLit(i32),
    FloatLit(f32),
    Symbol(String),
}
