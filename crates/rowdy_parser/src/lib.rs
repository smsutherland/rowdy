use rowdy_ast::{untyped::*, Token};
use rowdy_compiler::Compiler;
use rowdy_lexer::{
    token::{QualifiedToken as Token, QualifiedTokenType as TokenType},
    TokenIter,
};

pub fn parse_tokens(mut tokens: TokenIter, _compiler: &Compiler) -> Ast {
    parse(&mut tokens).unwrap()
}

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken {
        expected: &'static str,
        got: TokenType,
    },
    OutOfTokens,
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

impl Parse for Ast {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        let mut functions = Vec::new();
        while !tokens.is_empty() {
            functions.push(parse(tokens)?);
        }
        Ok(Self { functions })
    }
}

impl Parse for Function {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        let return_type: Type = parse(tokens)?;
        let name = parse(tokens)?;

        parse::<LParen>(tokens)?;
        let mut parameters = Vec::new();
        while let Ok(dec) = try_parse(tokens) {
            parameters.push(dec);
        }
        parse::<RParen>(tokens)?;
        let expr: BracedExpression = parse(tokens)?;

        Ok(Function {
            span: return_type.span().combine(expr.span()),
            return_type,
            name,
            parameters,
            expr,
        })
    }
}

impl Parse for Declaration {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        let typ: Type = parse(tokens)?;
        let name: Symbol = parse(tokens)?;
        Ok(Declaration {
            span: typ.span().combine(name.span),
            typ,
            name,
        })
    }
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

impl Parse for Statement {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        let first_symbol: Symbol = parse(tokens)?;
        if let Ok(second_symbol) = try_parse::<Symbol>(tokens) {
            // Declaration
            let dec = Declaration {
                span: first_symbol.span.combine(second_symbol.span),
                typ: first_symbol.into(),
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

impl Parse for Expression {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        match tokens.next().ok_or(ParseError::OutOfTokens)? {
            Token {
                typ: TokenType::SpecialChar(rowdy_lexer::token::SpecialChar::LBrace),
                ..
            } => Ok(Expression::Braced(parse(tokens)?)),
            Token {
                typ: TokenType::IntLit(x),
                span,
            } => Ok(Expression::IntLit(IntLit { span, value: x })),
            Token {
                typ: TokenType::FloatLit(x),
                span,
            } => Ok(Expression::FloatLit(FloatLit { span, value: x })),
            Token {
                typ: TokenType::Symbol(s),
                span,
            } => Ok(Expression::Symbol(Symbol { text: s, span })),
            other => Err(ParseError::UnexpectedToken {
                expected: "LBrace, IntLit, FloatLit, Symbol",
                got: other.typ,
            }),
        }
    }
}

impl Parse for Type {
    fn parse(tokens: &mut TokenIter) -> Result<Self> {
        Ok(parse::<Symbol>(tokens)?.into())
    }
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

macro_rules! parse_node {
    ($kind:ident :: $name:ident) => {
        impl Parse for $name {
            fn parse(tokens: &mut TokenIter) -> Result<Self> {
                match tokens.next() {
                    Some(Token {
                        typ: TokenType::$kind(rowdy_lexer::token::$kind::$name),
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
    };
}

macro_rules! special_char_parse {
    ($name:ident) => {
        parse_node! {SpecialChar::$name}
    };
}

special_char_parse! {LParen}
special_char_parse! {RParen}
special_char_parse! {LBrace}
special_char_parse! {RBrace}
special_char_parse! {LBracket}
special_char_parse! {RBracket}
special_char_parse! {Comma}

macro_rules! operator_parse {
    ($name:ident) => {
        parse_node! {Operator::$name}
    };
}

operator_parse! {Plus}
operator_parse! {PlusAssign}
operator_parse! {Increment}
operator_parse! {Sub}
operator_parse! {SubAssign}
operator_parse! {Decrement}
operator_parse! {Assign}
operator_parse! {Equals}

macro_rules! keyword_parse {
    ($name:ident) => {
        parse_node! {Keyword::$name}
    };
}

keyword_parse! {If}
keyword_parse! {Else}
keyword_parse! {While}
keyword_parse! {For}
keyword_parse! {Return}

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
