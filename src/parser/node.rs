//! Tokens similar to that of `lexer::token` but aranged into separate types rather than an enum.

use super::{parse, single_token::*, try_parse, Parse, ParseError, Result, Spanned};
use crate::Token;
use crate::{
    lexer::{
        token::{self, QualifiedToken as Token, QualifiedTokenType as TokenType},
        TokenIter,
    },
    location::Span,
};

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

        parse::<LParen>(tokens)?;
        let mut parameters = Vec::new();
        while let Ok(dec) = try_parse(tokens) {
            parameters.push(dec);
        }
        parse::<RParen>(tokens)?;
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
        let start_brace: LBrace = parse(tokens)?;
        let mut statements = Vec::new();
        while let Ok(statement) = try_parse(tokens) {
            statements.push(statement);
        }
        let end_brace: RBrace = parse(tokens)?;
        Ok(BracedExpression {
            statements,
            span: start_brace.span().combine(end_brace.span()),
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
            if try_parse::<Token![=]>(tokens).is_ok() {
                // Declaration w/ assignment
                let expr = parse(tokens)?;
                parse::<Token![;]>(tokens)?;
                Ok(Statement::Declaration(dec, Some(expr)))
            } else {
                parse::<Token![;]>(tokens)?;
                Ok(Statement::Declaration(dec, None))
            }
        } else if try_parse::<Token![=]>(tokens).is_ok() {
            // Assignment
            let expr = parse(tokens)?;
            parse::<Token![;]>(tokens)?;

            Ok(Statement::Assignment(first_symbol, expr))
        } else {
            // Function call
            parse::<LParen>(tokens)?;
            let mut exprs = Vec::new();
            loop {
                if try_parse::<RParen>(tokens).is_ok() {
                    break;
                }
                exprs.push(parse(tokens)?);
                if try_parse::<Token![,]>(tokens).is_err() {
                    parse::<RParen>(tokens)?;
                    break;
                }
            }
            parse::<Token![;]>(tokens)?;
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
            _ => Err(ParseError::UnexpectedToken),
        }
    }
}

impl Spanned for Expression {
    fn span(&self) -> Span {
        todo!()
    }
}
