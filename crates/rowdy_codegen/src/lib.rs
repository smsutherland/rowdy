use rowdy_ast::typed::*;
use rowdy_bytecode::{Bytecode, Instruction};

pub fn generate_bytecode(ast: &Ast) -> Bytecode {
    let mut gen = Generator::default();
    gen.visit(ast);
    gen.bytecode
}

#[derive(Debug, Default)]
struct Generator {
    bytecode: Bytecode,
}

impl Visit<Ast> for Generator {
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

impl Visit<Function> for Generator {
    type Output = ();

    fn visit(&mut self, node: &Function) -> Self::Output {
        self.visit(&node.expr);
    }
}

impl Visit<BracedExpression> for Generator {
    type Output = ();

    fn visit(&mut self, node: &BracedExpression) -> Self::Output {
        for stmt in &node.statements {
            self.visit(stmt);
        }
    }
}

impl Visit<Statement> for Generator {
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

impl Visit<Expression> for Generator {
    type Output = ();

    fn visit(&mut self, node: &Expression) -> Self::Output {
        match &node.inner {
            typed::ExpressionInner::Braced(_) => todo!(),
            typed::ExpressionInner::IntLit(lit) => self.bytecode.push(Instruction::Push(lit.value)),
            typed::ExpressionInner::FloatLit(_) => todo!(),
            typed::ExpressionInner::Symbol(_) => todo!(),
        };
    }
}

trait Visit<Node> {
    type Output;

    fn visit(&mut self, node: &Node) -> Self::Output;
}
