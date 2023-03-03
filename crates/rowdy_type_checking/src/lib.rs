use rowdy_ast::{base, typed, Spanned};
use rowdy_compiler::Compiler;
use rowdy_diagnostics as diagnostic;
use rowdy_types::{FnSignature, TypeID};
use std::collections::BTreeMap;

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
    functions: BTreeMap<String, FnSignature>,
    types: BTreeMap<String, TypeID>,
    next_key: TypeID,
    symbol_table: BTreeMap<String, TypeID>,
    compiler: &'a Compiler,
}

impl<'a> TypeChecker<'a> {
    fn new(compiler: &'a Compiler) -> Self {
        Self {
            functions: BTreeMap::new(),
            types: BTreeMap::new(),
            next_key: 0,
            symbol_table: BTreeMap::new(),
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

    fn visit(&mut self, node: &base::Ast) -> Self::Output {
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

        let mut result = typed::Ast::default();
        for func in &node.functions {
            result.functions.push(self.visit(func));
        }
        result
    }
}

impl Visit<base::Function> for TypeChecker<'_> {
    type Output = typed::Function;

    fn visit(&mut self, node: &base::Function) -> Self::Output {
        self.symbol_table.clear();
        for param in &node.parameters {
            let param_type_id = self.type_name_lookup(&param.typ.symbol.text);
            self.symbol_table
                .insert(param.name.text.clone(), param_type_id);
        }

        self.visit(&node.expr);

        typed::Function {
            span: node.span,
            return_type: self.visit(&node.return_type),
            name: self.visit(&node.name),
            parameters: self.visit(&node.parameters),
            expr: self.visit(&node.expr),
            signature: self
                .functions
                .get(&node.name.text)
                .expect("Visited function without first putting it in the functions map")
                .clone(),
        }
    }
}

impl Visit<base::BracedExpression> for TypeChecker<'_> {
    type Output = typed::BracedExpression;

    fn visit(&mut self, node: &base::BracedExpression) -> Self::Output {
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
                    let initialization_type_id = self.visit(initialization).typed;
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
                    let expr_type_id = self.visit(expression).typed;
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
        todo!()
    }
}

impl Visit<base::Expression> for TypeChecker<'_> {
    type Output = typed::Expression;

    fn visit(&mut self, node: &base::Expression) -> Self::Output {
        match node {
            base::Expression::Braced(braced_expr) => {
                // TODO: Make this into an .into() call by doing _more_ macros on the AST
                let visited = self.visit(braced_expr);
                typed::Expression {
                    typed: visited.typed,
                    inner: typed::ExpressionInner::Braced(visited),
                }
            }
            base::Expression::IntLit(int_lit) => {
                let visited = self.visit(int_lit);
                typed::Expression {
                    typed: visited.typed,
                    inner: typed::ExpressionInner::IntLit(visited),
                }
            }
            base::Expression::FloatLit(float_lit) => {
                let visited = self.visit(float_lit);
                typed::Expression {
                    typed: visited.typed,
                    inner: typed::ExpressionInner::FloatLit(visited),
                }
            }
            base::Expression::Symbol(symbol) => {
                let visited = self.visit(symbol);
                typed::Expression {
                    typed: visited.typed,
                    inner: typed::ExpressionInner::Symbol(visited),
                }
            }
        }
    }
}

impl Visit<base::IntLit> for TypeChecker<'_> {
    type Output = typed::IntLit;

    fn visit(&mut self, node: &base::IntLit) -> Self::Output {
        todo!()
    }
}

impl Visit<base::FloatLit> for TypeChecker<'_> {
    type Output = typed::FloatLit;

    fn visit(&mut self, node: &base::FloatLit) -> Self::Output {
        todo!()
    }
}

impl Visit<base::Symbol> for TypeChecker<'_> {
    type Output = typed::Symbol;

    fn visit(&mut self, node: &base::Symbol) -> Self::Output {
        todo!()
    }
}

impl Visit<base::Type> for TypeChecker<'_> {
    type Output = typed::Type;

    fn visit(&mut self, node: &base::Type) -> Self::Output {
        todo!()
    }
}

impl Visit<Vec<base::Declaration>> for TypeChecker<'_> {
    type Output = Vec<typed::Declaration>;

    fn visit(&mut self, node: &Vec<base::Declaration>) -> Self::Output {
        todo!()
    }
}

trait Visit<Node> {
    type Output;

    fn visit(&mut self, node: &Node) -> Self::Output;
}
