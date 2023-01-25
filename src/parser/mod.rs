use crate::lexer::TokenIter;
mod node;
use node::*;

#[derive(Debug)]
pub struct Program {
    functions: Vec<Function>,
}

pub type Ast = Program;

pub fn parse_tokens(tokens: TokenIter) -> Ast {
    let mut tokens = tokens;
    let mut functions = Vec::new();
    while let Ok(func) = parse(&mut tokens) {
        functions.push(func);
    }

    Program { functions }
}
