use std::iter::Peekable;

use crate::lexer::{
    token::{self, QualifiedToken, QualifiedTokenType},
    TokenIter,
};
mod node;
use node::*;

#[derive(Debug)]
pub struct Program {
    functions: Vec<Function>,
}

pub type Ast = Program;

macro_rules! match_type_peek {
    ($tokens:expr, $typ:pat) => {
        if let $typ = $tokens.peek().unwrap().typ {
            true
        } else {
            false
        }
    };
}

macro_rules! match_type {
    ($tokens:expr, $typ:pat) => {
        if let $typ = $tokens.peek().unwrap().typ {
            $tokens.next().unwrap()
        } else {
            panic!(
                "Expected {} but found {:?}",
                stringify!($typ),
                $tokens.peek().unwrap().typ
            );
            // TODO: Proper error handling
        }
    };
    ($tokens:expr, $typ:pat, $($var:ident),+) => {
        if let $typ = $tokens.next().unwrap().typ {
            ($($var),+)
        } else {
            panic!(
                "Expected {} but found {:?}",
                stringify!($typ),
                $tokens.peek().unwrap().typ
            );
            // TODO: Proper error handling
        }
    };
}

pub fn parse_tokens(tokens: TokenIter) -> Ast {
    let mut tokens = tokens;
    let mut functions = Vec::new();
    while !tokens.is_empty() {
        functions.push(Function::parse(&mut tokens).unwrap());
    }
    Program { functions }
}

fn declaration<'a, I>(tokens: &mut Peekable<I>) -> Declaration
where
    I: Iterator<Item = QualifiedToken>,
{
    let typ = match_type!(tokens, QualifiedTokenType::Symbol(x), x);
    let name = match_type!(tokens, QualifiedTokenType::Symbol(x), x);
    Declaration { typ, name }
}

fn braced_expression<'a, I>(tokens: &mut Peekable<I>) -> BracedExpression
where
    I: Iterator<Item = QualifiedToken>,
{
    use QualifiedTokenType::*;
    match_type!(tokens, SpecialChar(token::SpecialChar::LBrace));
    let mut statements = Vec::new();
    while !match_type_peek!(tokens, SpecialChar(token::SpecialChar::RBrace)) {
        statements.push(statement(tokens));
    }
    match_type!(tokens, SpecialChar(token::SpecialChar::RBrace));
    BracedExpression { statements }
}

fn statement<'a, I>(tokens: &mut Peekable<I>) -> Statement
where
    I: Iterator<Item = QualifiedToken>,
{
    use QualifiedTokenType::*;
    if match_type_peek!(tokens, Symbol(_)) {
        let target = if let Symbol(s) = tokens.next().unwrap().typ {
            s
        } else {
            unsafe {
                std::hint::unreachable_unchecked();
            }
        };
        if match_type_peek!(tokens, Symbol(_)) {
            // Declaration
            // let dec = declaration(tokens);
            let name = if let Symbol(s) = tokens.next().unwrap().typ {
                s
            } else {
                unsafe {
                    std::hint::unreachable_unchecked();
                }
            };
            let dec = Declaration { typ: target, name };
            if match_type_peek!(tokens, Operator(token::Operator::Assign)) {
                match_type!(tokens, Operator(token::Operator::Assign));
                let expr = expression(tokens);
                match_type!(tokens, End);
                Statement::Declaration(dec, Some(expr))
            } else {
                match_type!(tokens, End);
                Statement::Declaration(dec, None)
            }
        } else if match_type_peek!(tokens, Operator(token::Operator::Assign)) {
            // Assignment
            match_type!(tokens, Operator(token::Operator::Assign));

            let expr = expression(tokens);
            match_type!(tokens, End);

            Statement::Assignment(target, expr)
        } else if match_type_peek!(tokens, SpecialChar(token::SpecialChar::LParen)) {
            // Function call
            match_type!(tokens, SpecialChar(token::SpecialChar::LParen));
            let mut exprs = Vec::new();
            loop {
                if match_type_peek!(tokens, SpecialChar(token::SpecialChar::RParen)) {
                    break;
                }
                exprs.push(expression(tokens));
                if match_type_peek!(tokens, SpecialChar(token::SpecialChar::Comma)) {
                    match_type!(tokens, SpecialChar(token::SpecialChar::Comma));
                }
            }
            match_type!(tokens, SpecialChar(token::SpecialChar::RParen));
            match_type!(tokens, End);
            Statement::FunctionCall(target, exprs)
        } else {
            panic!("{:?}", tokens.peek().unwrap());
        }
    } else {
        panic!("{:?}", tokens.peek().unwrap());
    }
}

fn expression<'a, I>(tokens: &mut Peekable<I>) -> Expression
where
    I: Iterator<Item = QualifiedToken>,
{
    match tokens.next().unwrap().typ {
        QualifiedTokenType::SpecialChar(token::SpecialChar::LBrace) => {
            Expression::Braced(braced_expression(tokens))
        }
        QualifiedTokenType::IntLit(x) => Expression::IntLit(x),
        QualifiedTokenType::FloatLit(x) => Expression::FloatLit(x),
        QualifiedTokenType::Symbol(s) => Expression::Symbol(s),
        _ => panic!(),
    }
}
