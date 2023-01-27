pub trait Visit<AST> {
    fn visit(&mut self, node: &mut AST);
}
