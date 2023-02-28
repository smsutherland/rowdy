use rowdy_location::Span;

pub trait Spanned {
    fn span(&self) -> Span;
}

#[macro_export]
macro_rules! Token {
    [,] => {$crate::Comma};
    [;] => {$crate::End};
    [+] => {$crate::Plus};
    [+=] => {$crate::PlusAssign};
    [++] => {$crate::Increment};
    [-] => {$crate::Sub};
    [-=] => {$crate::SubAssign};
    [--] => {$crate::Decrement};
    [=] => {$crate::Assign};
    [==] => {$crate::Equals};
}

macro_rules! make_node {
    ($name:ident) => {
        #[derive(Debug)]
        pub struct $name {
            pub span: Span,
        }

        impl Spanned for $name {
            fn span(&self) -> Span {
                self.span
            }
        }
    };
}

make_node! {LParen}
make_node! {RParen}
make_node! {LBrace}
make_node! {RBrace}
make_node! {LBracket}
make_node! {RBracket}
make_node! {Comma}
make_node! {End}

make_node! {Plus}
make_node! {PlusAssign}
make_node! {Increment}
make_node! {Sub}
make_node! {SubAssign}
make_node! {Decrement}
make_node! {Assign}
make_node! {Equals}

make_node! {If}
make_node! {Else}
make_node! {While}
make_node! {For}
make_node! {Return}

#[rowdy_macros::ast(typed)]
pub mod base {
    pub use super::*;
    use rowdy_location::Span;

    #[derive(Debug)]
    pub struct Program {
        pub functions: Vec<Function>,
    }

    pub type Ast = Program;

    #[ast::typed{pub typed: ::rowdy_types::FnSignature}]
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

    #[ast::typed{pub typed: ::rowdy_types::TypeID}]
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

    #[ast::typed{pub typed: ::rowdy_types::TypeID}]
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

    #[ast::typed{pub typed: ::rowdy_types::TypeID}]
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

    #[ast::typed{pub typed: ::rowdy_types::TypeID}]
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

    #[ast::typed{pub typed: ::rowdy_types::TypeID}]
    #[derive(Debug)]
    pub enum Expression {
        Braced(BracedExpression),
        IntLit(IntLit),
        FloatLit(FloatLit),
        Symbol(Symbol),
    }

    #[ast::base]
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

    #[ast::typed]
    impl Spanned for Expression {
        fn span(&self) -> Span {
            match &self.inner {
                ExpressionInner::Braced(braced) => braced.span,
                ExpressionInner::IntLit(int_lit) => int_lit.span,
                ExpressionInner::FloatLit(float_lit) => float_lit.span,
                ExpressionInner::Symbol(symbol) => symbol.span,
            }
        }
    }

    #[ast::typed{pub typ: ::rowdy_types::TypeID}]
    #[derive(Debug)]
    pub struct Type {
        pub symbol: Symbol,
    }

    #[ast::base]
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

    #[ast::typed{pub typ: ::rowdy_types::TypeID}]
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
}
