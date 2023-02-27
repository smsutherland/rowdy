use rowdy_ast::{base, typed, Spanned};
use rowdy_compiler::Compiler;
use rowdy_diagnostics as diagnostic;
use rowdy_types::{FnSignature, TypeID};
use std::collections::HashMap;

pub fn type_check(ast: &base::Ast, compiler: &Compiler) -> typed::Ast {
    let mut checker = TypeChecker::new(compiler);
    checker.visit(ast)
}

// 1. go through and find all type definitions
// 2. give each fully qualified type an id
// 2.1. primitive types already have a type id
// 3. store types by their internal id

#[derive(Debug)]
struct TypeChecker<'a> {
    functions: HashMap<String, FnSignature>,
    types: HashMap<String, TypeID>,
    next_key: TypeID,
    symbol_table: HashMap<String, TypeID>,
    compiler: &'a Compiler,
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

impl Visit<base::Ast> for TypeChecker<'_> {
    type Output = typed::Ast;

    fn visit(&mut self, node: &base::Ast) -> typed::Ast {
        self.functions = node
            .functions
            .iter()
            .map(|function| {
                (
                    function.name.text.clone(),
                    FnSignature {
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
        todo!()
    }
}

impl Visit<base::Function> for TypeChecker<'_> {
    type Output = ();

    fn visit(&mut self, node: &base::Function) {
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

impl Visit<base::BracedExpression> for TypeChecker<'_> {
    type Output = TypeID;

    fn visit(&mut self, node: &base::BracedExpression) -> TypeID {
        for statement in &node.statements {
            // println!("{statement:#?}");
            match statement {
                base::Statement::Declaration(declaration, None) => {
                    let dec_type_id = self.type_name_lookup(&declaration.typ.symbol.text);
                    self.symbol_table
                        .insert(declaration.name.text.clone(), dec_type_id);
                }
                base::Statement::Declaration(declaration, Some(initialization)) => {
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
                base::Statement::Assignment(target, expression) => {
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
                base::Statement::FunctionCall(_name, _params) => todo!(),
            }
        }
        0
    }
}

impl Visit<base::Expression> for TypeChecker<'_> {
    type Output = TypeID;

    fn visit(&mut self, node: &base::Expression) -> TypeID {
        match node {
            base::Expression::Braced(braced_expr) => self.visit(braced_expr),
            base::Expression::IntLit(_) => self.type_name_lookup("int"),
            base::Expression::FloatLit(_) => self.type_name_lookup("float"),
            base::Expression::Symbol(symbol) => *self.symbol_table.get(&symbol.text).unwrap(),
        }
    }
}

trait Visit<Node> {
    type Output;

    fn visit(&mut self, node: &Node) -> Self::Output;
}
