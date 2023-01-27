use crate::ast::*;
use std::collections::HashMap;

pub trait Visit<AST> {
    fn visit(&mut self, node: &mut AST);
}

// 1. go through and find all type definnitions
// 2. give each fully qualified type an id
// 2.1. primitive types already have a type id
// 3. store types by their internal id

#[derive(Debug)]
struct FunctionInfo {
    return_type: String,
    parameter_types: Vec<String>,
}

#[derive(Debug, Default)]
pub struct TypeChecker {
    functions: HashMap<String, FunctionInfo>,
}

impl Visit<Ast> for TypeChecker {
    fn visit(&mut self, node: &mut Ast) {
        self.functions = node
            .functions
            .iter()
            .map(|function| {
                (
                    function.name.text.clone(),
                    FunctionInfo {
                        return_type: function.return_type.symbol.text.clone(),
                        parameter_types: function
                            .parameters
                            .iter()
                            .map(|dec| dec.typ.symbol.text.clone())
                            .collect(),
                    },
                )
            })
            .collect()
    }
}
