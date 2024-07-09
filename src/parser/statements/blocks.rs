use super::Statement;
#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Box<dyn Statement>>,
}
impl Statement for Block {
    fn generate_code(&self, codegen: &mut crate::codegen::CodeGen) {
        codegen.increase_scope();
        for stmt in self.statements.iter() {
            stmt.generate_code(codegen)
        }
        codegen.decrease_scope();
    }
    fn desugar(&self) -> Box<dyn Statement> {
        let mut new_stmts = vec![];
        for stmt in self.statements.iter() {
            new_stmts.push(stmt.desugar());
        }
        Box::new(Block {
            statements: new_stmts
        })
    }
}
