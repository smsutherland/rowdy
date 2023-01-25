//! Tokens similar to that of `lexer::token` but aranged into separate types rather than an enum.

use crate::{
    lexer::{
        token::{self, QualifiedToken as Token, QualifiedTokenType as TokenType},
        TokenIter,
    },
    location::Span,
};

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken,
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
    #[inline]
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
        let return_type: Symbol = parse(tokens)?;
        let name = parse(tokens)?;

        let Some(Token {
            typ: TokenType::SpecialChar(token::SpecialChar::LParen),
            ..
        }) = tokens.next() else {
            return Err(ParseError::UnexpectedToken);
        };
        let mut parameters = Vec::new();
        while {
            match tokens.clone().next() {
                Some(Token {
                    typ: TokenType::Symbol(_),
                    ..
                }) => true,
                _ => false,
            }
        } {
            parameters.push(parse(tokens)?);
        }
        let Some(Token {
            typ: TokenType::SpecialChar(token::SpecialChar::RParen),
            ..
        }) = tokens.next() else {
            return Err(ParseError::UnexpectedToken);
        };
        let expr: BracedExpression = parse(tokens)?;

        Ok(Function {
            span: return_type.span.combine(expr.span()),
            return_type,
            name,
            parameters,
            expr,
        })
    }
}

impl Spanned for Function {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug)]
pub struct Declaration {
    span: Span,
    typ: Symbol,
    name: Symbol,
}

impl Parse for Declaration {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        let typ = Symbol::parse(tokens)?;
        let name = Symbol::parse(tokens)?;
        Ok(Declaration {
            span: typ.span.combine(name.span),
            typ,
            name,
        })
    }
}

impl Spanned for Declaration {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug)]
pub struct BracedExpression {
    span: Span,
    statements: Vec<Statement>,
}

impl Parse for BracedExpression {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        let Some(Token {
            typ: TokenType::SpecialChar(token::SpecialChar::LBrace),
            span: start_span,
        }) = tokens.next() else {
            return Err(ParseError::UnexpectedToken);
        };
        let mut statements = Vec::new();
        while let Ok(statement) = try_parse(tokens) {
            statements.push(statement);
        }
        let Some(Token {
            typ: TokenType::SpecialChar(token::SpecialChar::RBrace),
            span: end_span,
        }) = tokens.next() else {
            return Err(ParseError::UnexpectedToken);
        };

        Ok(BracedExpression {
            statements,
            span: start_span.combine(end_span),
        })
    }
}

impl Spanned for BracedExpression {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug)]
pub enum Statement {
    Declaration(Declaration, Option<Expression>),
    Assignment(Symbol, Expression),
    FunctionCall(Symbol, Vec<Expression>),
}

impl Parse for Statement {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        let first_symbol: Symbol = parse(tokens)?;
        if let Ok(second_symbol) = try_parse::<Symbol>(tokens) {
            // Declaration
            let dec = Declaration {
                span: first_symbol.span.combine(second_symbol.span),
                typ: first_symbol,
                name: second_symbol,
            };
            if let Some(Token {
                typ: TokenType::Operator(token::Operator::Assign),
                ..
            }) = tokens.clone().next()
            {
                let expr = parse(tokens)?;
                let Some(Token {
                        typ: TokenType::End,
                        ..
                    }) = tokens.next() else {
                        return Err(ParseError::UnexpectedToken);
                    };
                Ok(Statement::Declaration(dec, Some(expr)))
            } else {
                let Some(Token {
                        typ: TokenType::End,
                        ..
                    }) = tokens.next() else {
                        return Err(ParseError::UnexpectedToken);
                    };
                Ok(Statement::Declaration(dec, None))
            }
        } else if let Some(Token {
            typ: TokenType::Operator(token::Operator::Assign),
            ..
        }) = tokens.clone().next()
        {
            // Assignment
            tokens.next();

            let expr = parse(tokens)?;
            let Some(Token {
                typ: TokenType::End,
                ..
            }) = tokens.next() else {
                return Err(ParseError::UnexpectedToken)
            };

            Ok(Statement::Assignment(first_symbol, expr))
        } else {
            // Function call
            let Some(Token {
                typ: TokenType::SpecialChar(token::SpecialChar::LParen),
                ..
            }) = tokens.next() else {return Err(ParseError::UnexpectedToken)};
            let mut exprs = Vec::new();
            loop {
                if let Some(Token {
                    typ: TokenType::SpecialChar(token::SpecialChar::RParen),
                    ..
                }) = tokens.clone().next()
                {
                    break;
                }
                exprs.push(parse(tokens)?);
                if let Some(Token {
                    typ: TokenType::SpecialChar(token::SpecialChar::Comma),
                    ..
                }) = tokens.clone().next()
                {
                    tokens.next();
                }
            }
            let Some(Token {
                typ: TokenType::SpecialChar(token::SpecialChar::RParen),
                ..
            }) = tokens.next() else {return Err(ParseError::UnexpectedToken)};
            let Some(Token {
                typ: TokenType::End,
                ..
            }) = tokens.next() else {return Err(ParseError::UnexpectedToken)};
            Ok(Statement::FunctionCall(first_symbol, exprs))
        }
    }
}

impl Spanned for Statement {
    fn span(&self) -> Span {
        todo!()
    }
}

#[derive(Debug)]
pub enum Expression {
    Braced(BracedExpression),
    IntLit(i32),
    FloatLit(f32),
    Symbol(String),
}

impl Parse for Expression {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next().ok_or(ParseError::UnexpectedToken)? {
            Token {
                typ: TokenType::SpecialChar(token::SpecialChar::LBrace),
                ..
            } => Ok(Expression::Braced(parse(tokens)?)),
            Token {
                typ: TokenType::IntLit(x),
                ..
            } => Ok(Expression::IntLit(x)),
            Token {
                typ: TokenType::FloatLit(x),
                ..
            } => Ok(Expression::FloatLit(x)),
            Token {
                typ: TokenType::Symbol(s),
                ..
            } => Ok(Expression::Symbol(s)),
            _ => panic!(),
        }
    }
}

impl Spanned for Expression {
    fn span(&self) -> Span {
        todo!()
    }
}
