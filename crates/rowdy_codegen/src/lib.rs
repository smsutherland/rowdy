use rowdy_bytecode::{Bytecode, Instruction};
use rowdy_ast::*;

pub fn generate_bytecode(ast: &Ast) -> Bytecode {
    let mut code = Bytecode::default();
    code.visit(ast);
    code
}

impl Visit<Ast> for Bytecode {
    type Output = ();

    fn visit(&mut self, node: &Ast) -> Self::Output {
        // NOTE: Only can handle a single function right now
        self.visit(
            node.functions
                .get(0)
                .expect("There was no function to compile to bytecode"),
        );
    }
}

impl Visit<Function> for Bytecode {
    type Output = ();

    fn visit(&mut self, node: &Function) -> Self::Output {
        self.visit(&node.expr);
    }
}

impl Visit<BracedExpression> for Bytecode {
    type Output = ();

    fn visit(&mut self, node: &BracedExpression) -> Self::Output {
        for stmt in &node.statements {
            self.visit(stmt);
        }
    }
}

impl Visit<Statement> for Bytecode {
    type Output = ();

    fn visit(&mut self, node: &Statement) -> Self::Output {
        match node {
            Statement::Declaration(_, Some(expr)) => self.visit(expr),
            Statement::Declaration(_, None) => todo!(),
            Statement::Assignment(_, _) => todo!(),
            Statement::FunctionCall(_, _) => todo!(),
        }
    }
}

impl Visit<Expression> for Bytecode {
    type Output = ();

    fn visit(&mut self, node: &Expression) -> Self::Output {
        match node {
            Expression::Braced(_) => todo!(),
            Expression::IntLit(lit) => self.push(Instruction::Push(lit.value)),
            Expression::FloatLit(_) => todo!(),
            Expression::Symbol(_) => todo!(),
        };
    }
}

trait Visit<Node> {
    type Output;

    fn visit(&mut self, node: &Node) -> Self::Output;
}
