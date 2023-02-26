pub use super::*;
use rowdy_location::Span;

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
}

pub type Ast = Program;

#[derive(Debug)]
pub struct Function {
    pub span: Span,
    pub return_type: Type,
    pub name: Symbol,
    pub parameters: Vec<Declaration>,
    pub expr: BracedExpression,
}

impl Spanned for Function {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug)]
pub struct Declaration {
    pub span: Span,
    pub typ: Type,
    pub name: Symbol,
}

impl Spanned for Declaration {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug)]
pub struct BracedExpression {
    pub span: Span,
    pub statements: Vec<Statement>,
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

impl Spanned for Statement {
    fn span(&self) -> Span {
        match self {
            Statement::Declaration(decl, Some(expr)) => decl.span.combine(expr.span()),
            Statement::Declaration(decl, None) => decl.span,
            Statement::Assignment(symbol, expr) => symbol.span.combine(expr.span()),
            Statement::FunctionCall(symbol, exprs) => {
                let mut span = symbol.span;
                if let Some(last) = exprs.last() {
                    span = span.combine(last.span())
                }
                span
            }
        }
    }
}

#[derive(Debug)]
pub struct IntLit {
    pub span: Span,
    pub value: i32,
}

impl Spanned for IntLit {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug)]
pub struct FloatLit {
    pub span: Span,
    pub value: f32,
}

impl Spanned for FloatLit {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug)]
pub enum Expression {
    Braced(BracedExpression),
    IntLit(IntLit),
    FloatLit(FloatLit),
    Symbol(Symbol),
}

impl Spanned for Expression {
    fn span(&self) -> Span {
        match self {
            Expression::Braced(braced) => braced.span,
            Expression::IntLit(int_lit) => int_lit.span,
            Expression::FloatLit(float_lit) => float_lit.span,
            Expression::Symbol(symbol) => symbol.span,
        }
    }
}

#[derive(Debug)]
pub struct Type {
    pub symbol: Symbol,
}

impl From<Symbol> for Type {
    fn from(symbol: Symbol) -> Self {
        Self { symbol }
    }
}

impl Spanned for Type {
    fn span(&self) -> Span {
        self.symbol.span
    }
}

#[derive(Debug)]
pub struct Symbol {
    pub text: String,
    pub span: Span,
}

impl Spanned for Symbol {
    fn span(&self) -> Span {
        self.span
    }
}
