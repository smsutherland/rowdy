use std::iter::Peekable;

use crate::lexer::token::{self, QualifiedToken, QualifiedTokenType};
use crate::lexer::Location;

pub trait ASTNode {
    fn loc(&self) -> Location;
}

#[derive(Debug)]
pub struct Program {
    functions: Vec<Function>,
}

impl ASTNode for Program {
    fn loc(&self) -> Location {
        todo!("program loc")
    }
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

pub fn parse_tokens(tokens: impl Iterator<Item = QualifiedToken>) -> Ast {
    let mut tokens = tokens.peekable();
    let mut functions = Vec::new();
    while tokens.peek().is_some() {
        functions.push(function(&mut tokens));
    }
    Program { functions }
}

#[derive(Debug)]
pub struct Function {
    loc: Location,
    return_type: String,
    name: String,
    parameters: Vec<Declaration>,
    expr: BracedExpression,
}

impl<'a> ASTNode for Function {
    fn loc(&self) -> Location {
        self.loc
    }
}

fn function<'a, I>(tokens: &mut Peekable<I>) -> Function
where
    I: Iterator<Item = QualifiedToken>,
{
    use QualifiedTokenType::*;
    let loc = tokens.peek().unwrap().span.start;
    let return_type = match_type!(tokens, Symbol(t), t);
    let name = match_type!(tokens, Symbol(s), s);
    match_type!(tokens, SpecialChar(token::SpecialChar::LParen));
    let mut parameters = Vec::new();
    while !match_type_peek!(tokens, SpecialChar(token::SpecialChar::RParen)) {
        parameters.push(declaration(tokens));
    }
    match_type!(tokens, SpecialChar(token::SpecialChar::RParen));

    let expr = braced_expression(tokens);

    Function {
        loc,
        return_type,
        name,
        parameters,
        expr,
    }
}

#[derive(Debug)]
pub struct Declaration {
    typ: String,
    name: String,
}

impl ASTNode for Declaration {
    fn loc(&self) -> Location {
        todo!("declaration loc")
    }
}

fn declaration<'a, I>(tokens: &mut Peekable<I>) -> Declaration
where
    I: Iterator<Item = QualifiedToken>,
{
    let typ = match_type!(tokens, QualifiedTokenType::Symbol(x), x);
    let name = match_type!(tokens, QualifiedTokenType::Symbol(x), x);
    Declaration { typ, name }
}

#[derive(Debug)]
struct BracedExpression {
    statements: Vec<Statement>,
}

impl ASTNode for BracedExpression {
    fn loc(&self) -> Location {
        todo!("bracedExpression loc")
    }
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

#[derive(Debug)]
enum Statement {
    Declaration(Declaration, Option<Expression>),
    Assignment(String, Expression),
    FunctionCall(String, Vec<Expression>),
}

impl ASTNode for Statement {
    fn loc(&self) -> Location {
        todo!("statement loc")
    }
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

#[derive(Debug)]
enum Expression {
    Braced(BracedExpression),
    IntLit(i32),
    FloatLit(f32),
    Symbol(String),
}

impl ASTNode for Expression {
    fn loc(&self) -> Location {
        todo!("expression loc")
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
