use crate::diagnostic;
use crate::visit::Visit;
use crate::{ast::*, Compiler};
use std::collections::HashMap;

pub fn type_check(ast: &mut Ast, compiler: &Compiler) {
    let mut checker = TypeChecker::new(compiler);
    checker.visit(ast);
    // dbg!(checker);
}

// 1. go through and find all type definitions
// 2. give each fully qualified type an id
// 2.1. primitive types already have a type id
// 3. store types by their internal id

type TypeID = u32;

#[derive(Debug)]
struct TypeChecker<'a> {
    functions: HashMap<String, FunctionInfo>,
    types: HashMap<String, TypeID>,
    next_key: TypeID,
    symbol_table: HashMap<String, TypeID>,
    compiler: &'a Compiler,
}

#[derive(Debug)]
struct FunctionInfo {
    return_type: TypeID,
    parameter_types: Vec<TypeID>,
}

impl<'a> TypeChecker<'a> {
    fn new(compiler: &'a Compiler) -> Self {
        Self {
            functions: HashMap::new(),
            types: HashMap::new(),
            next_key: 0,
            symbol_table: HashMap::new(),
            compiler,
        }
    }

    fn type_name_lookup(&mut self, name: &str) -> TypeID {
        match self.types.get(name) {
            Some(id) => *id,
            None => {
                let key = self.next_key;
                self.types.insert(name.to_owned(), key);
                self.next_key += 1;
                key
            }
        }
    }
}

impl Visit<Ast> for TypeChecker<'_> {
    type Output = ();

    fn visit(&mut self, node: &Ast) {
        self.functions = node
            .functions
            .iter()
            .map(|function| {
                (
                    function.name.text.clone(),
                    FunctionInfo {
                        return_type: self.type_name_lookup(&function.return_type.symbol.text),
                        parameter_types: function
                            .parameters
                            .iter()
                            .map(|dec| &dec.typ.symbol.text)
                            .map(|name| self.type_name_lookup(name))
                            .collect(),
                    },
                )
            })
            .collect();

        for func in &node.functions {
            self.visit(func);
        }
    }
}

impl Visit<Function> for TypeChecker<'_> {
    type Output = ();

    fn visit(&mut self, node: &Function) {
        self.symbol_table.clear();
        for param in &node.parameters {
            let param_type_id = self.type_name_lookup(&param.typ.symbol.text);
            self.symbol_table
                .insert(param.name.text.clone(), param_type_id);
        }

        self.visit(&node.expr);

        // println!("{:?}", self.symbol_table);
    }
}

impl Visit<BracedExpression> for TypeChecker<'_> {
    type Output = TypeID;

    fn visit(&mut self, node: &BracedExpression) -> TypeID {
        for statement in &node.statements {
            // println!("{statement:#?}");
            match statement {
                Statement::Declaration(declaration, None) => {
                    let dec_type_id = self.type_name_lookup(&declaration.typ.symbol.text);
                    self.symbol_table
                        .insert(declaration.name.text.clone(), dec_type_id);
                }
                Statement::Declaration(declaration, Some(initialization)) => {
                    let dec_type_id = self.type_name_lookup(&declaration.typ.symbol.text);
                    self.symbol_table
                        .insert(declaration.name.text.clone(), dec_type_id);
                    let initialization_type_id = self.visit(initialization);
                    if dec_type_id != initialization_type_id {
                        diagnostic::print_error(
                            statement.span(),
                            diagnostic::ErrorKind::E0000,
                            self.compiler,
                        )
                    }
                }
                Statement::Assignment(target, expression) => {
                    let target_type_id = self.symbol_table.get(&target.text).copied();
                    let expr_type_id = self.visit(expression);
                    if target_type_id != Some(expr_type_id) {
                        diagnostic::print_error(
                            statement.span(),
                            diagnostic::ErrorKind::E0000,
                            self.compiler,
                        )
                    }
                }
                Statement::FunctionCall(_name, _params) => todo!(),
            }
        }
        0
    }
}

impl Visit<Expression> for TypeChecker<'_> {
    type Output = TypeID;

    fn visit(&mut self, node: &Expression) -> TypeID {
        match node {
            Expression::Braced(braced_expr) => self.visit(braced_expr),
            Expression::IntLit(_) => self.type_name_lookup("int"),
            Expression::FloatLit(_) => self.type_name_lookup("float"),
            Expression::Symbol(symbol) => *self.symbol_table.get(&symbol.text).unwrap(),
        }
    }
}
