pub trait Visit<AST> {
    type Output;

    fn visit(&mut self, node: &AST) -> Self::Output;
}
