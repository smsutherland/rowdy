trait Visit<AST> {
    fn visit(&self, node: &AST);

    fn visit_mut(&mut self, node: &mut AST) {
        self.visit(node)
    }
}
